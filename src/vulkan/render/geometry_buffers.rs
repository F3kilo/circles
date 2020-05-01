use crate::vulkan::base::physical_device::PhysicalDevice;
use crate::vulkan::base::VulkanBase;
use crate::vulkan::render::vertex::Vertex;
use ash::util::Align;
use ash::version::DeviceV1_0;
use ash::vk;
use glam::Vec4;
use slog::Logger;
use std::os::raw::c_void;

pub struct GeometryBuffers {
    vertices: vk::Buffer,
    indices: vk::Buffer,
    memory: vk::DeviceMemory,
    memory_size: vk::DeviceSize,
    index_buffer_offset: vk::DeviceSize,
    logger: Logger,
}

impl GeometryBuffers {
    pub fn new(base: &VulkanBase, logger: Logger) -> Self {
        let vk_device = base.get_device().get_vk_device();
        let vertices = Self::create_vertex_buffer(vk_device);
        let indices = Self::create_index_buffer(vk_device);

        let (mem_req, index_buffer_offset) =
            Self::get_memory_requirements(vertices, indices, vk_device);
        let memory_size = mem_req.size;
        let pdevice = base.get_physical_device();
        let memory = Self::allocate_memory(pdevice, vk_device, &mem_req);
        Self {
            vertices,
            indices,
            memory,
            memory_size,
            index_buffer_offset,
            logger,
        }
    }

    pub fn write_triangle(&mut self, vk_device: &ash::Device) {
        let mem_ptr = self.map_memory(0, self.memory_size, vk_device);

        let vertices = Self::triangle_vertices();
        let mut vert_align = unsafe {
            Align::new(
                mem_ptr,
                std::mem::align_of::<Vertex>() as vk::DeviceSize,
                self.memory_size,
            )
        };
        vert_align.copy_from_slice(&vertices);

        let mem_ptr = unsafe { (mem_ptr as *mut u8).offset(self.index_buffer_offset as isize) };
        let indices = [0u32, 1, 2];
        let mut inds_align = unsafe {
            Align::new(
                mem_ptr as *mut c_void,
                std::mem::align_of::<u32>() as vk::DeviceSize,
                self.memory_size,
            )
        };
        inds_align.copy_from_slice(&indices);
        unsafe {
            vk_device.unmap_memory(self.memory);
        }
    }

    fn map_memory(
        &self,
        from: vk::DeviceSize,
        size: vk::DeviceSize,
        vk_device: &ash::Device,
    ) -> *mut c_void {
        unsafe { vk_device.map_memory(self.memory, from, size, vk::MemoryMapFlags::empty()) }
            .expect("Can't map geometry buffers memory.")
    }

    fn triangle_vertices() -> [Vertex; 3] {
        [
            Vertex {
                position: Vec4::new(-1., 1., 0., 1.),
                color: Vec4::new(1., 0., 0., 1.),
            },
            Vertex {
                position: Vec4::new(1., 1., 0., 1.),
                color: Vec4::new(0., 1., 0., 1.),
            },
            Vertex {
                position: Vec4::new(0., -1., 0., 1.),
                color: Vec4::new(0., 0., 1., 1.),
            },
        ]
    }

    fn allocate_memory(
        pdevice: &PhysicalDevice,
        vk_device: &ash::Device,
        mem_req: &vk::MemoryRequirements,
    ) -> vk::DeviceMemory {
        let memory_type_index = pdevice
            .find_memorytype_index(mem_req, vk::MemoryPropertyFlags::HOST_VISIBLE)
            .expect("Can't find suit memory type fo geometry buffers.");
        let allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(mem_req.size)
            .memory_type_index(memory_type_index);

        unsafe { vk_device.allocate_memory(&allocate_info, None) }
            .expect("Can't allocate memory for geometry buffers.")
    }

    fn get_memory_requirements(
        vertex_buffer: vk::Buffer,
        index_buffer: vk::Buffer,
        vk_device: &ash::Device,
    ) -> (vk::MemoryRequirements, vk::DeviceSize) {
        let vb_req = unsafe { vk_device.get_buffer_memory_requirements(vertex_buffer) };
        let ib_req = unsafe { vk_device.get_buffer_memory_requirements(index_buffer) };
        let alignment = primapalooza::least_common_multiple(
            vb_req.alignment as usize,
            ib_req.alignment as usize,
        ) as u64;
        let vb_aligned_size = Self::aligned_size(vb_req.size, alignment);
        let ib_aligned_size = Self::aligned_size(ib_req.size, alignment);
        let size = vb_aligned_size + ib_aligned_size;
        let memory_type_bits = vb_req.memory_type_bits | ib_req.memory_type_bits;
        (
            vk::MemoryRequirements {
                size,
                alignment,
                memory_type_bits,
            },
            vb_aligned_size,
        )
    }

    fn aligned_size(size: u64, align: u64) -> u64 {
        if align == 0 {
            return size;
        }
        let mut parts = size / align;
        if size % align > 0 {
            parts += 1;
        }
        parts * align
    }

    fn create_vertex_buffer(vk_device: &ash::Device) -> vk::Buffer {
        let size = (std::mem::size_of::<Vertex>() * 3) as u64;
        Self::create_buffer(size, vk::BufferUsageFlags::VERTEX_BUFFER, vk_device)
    }

    fn create_index_buffer(vk_device: &ash::Device) -> vk::Buffer {
        let size = (std::mem::size_of::<u32>() * 3) as u64;
        Self::create_buffer(size, vk::BufferUsageFlags::INDEX_BUFFER, vk_device)
    }

    pub fn create_buffer(
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        vk_device: &ash::Device,
    ) -> vk::Buffer {
        let create_info = vk::BufferCreateInfo::builder().size(size).usage(usage);
        unsafe { vk_device.create_buffer(&create_info, None) }.expect("Can't create buffer")
    }

    pub fn destroy(&mut self, vk_device: &ash::Device) {
        debug!(self.logger, "Geometry buffers destroy() called");
        unsafe {
            vk_device.destroy_buffer(self.vertices, None);
            debug!(self.logger, "\tVertex buffer destroyed");
            vk_device.destroy_buffer(self.indices, None);
            debug!(self.logger, "\tIndex buffer destroyed");
            vk_device.free_memory(self.memory, None);
            debug!(self.logger, "\tGeometry buffers memory freed");
        }
    }
}
