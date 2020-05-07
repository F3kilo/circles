use crate::vulkan::base::VulkanBase;
use crate::vulkan::present::VulkanPresent;
use crate::vulkan::render::render_pass::RenderPass;
use crate::vulkan::render::vertex::Vertex;
use ash::version::DeviceV1_0;
use ash::vk;
use slog::Logger;
use std::ffi::CString;
use std::io::Cursor;

macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}

pub struct Pipeline {
    pipeline: vk::Pipeline,
    layout: vk::PipelineLayout,
    logger: Logger,
}

impl Pipeline {
    pub fn new(
        base: &VulkanBase,
        presenter: &VulkanPresent,
        logger: Logger,
        render_pass: &RenderPass,
    ) -> Self {
        let vk_device = base.get_device().get_vk_device();

        let (vert_shader, frag_shader) = Self::load_shaders(vk_device);

        let layout = Self::create_layout(vk_device);

        let shader_entry_name = CString::new("main").unwrap();
        let shader_stage_create_infos = [
            vk::PipelineShaderStageCreateInfo {
                module: vert_shader,
                p_name: shader_entry_name.as_ptr(),
                stage: vk::ShaderStageFlags::VERTEX,
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                module: frag_shader,
                p_name: shader_entry_name.as_ptr(),
                stage: vk::ShaderStageFlags::FRAGMENT,
                ..Default::default()
            },
        ];

        let vertex_input_binding_descriptions = [vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }];
        let vertex_input_attribute_descriptions = [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(Vertex, position) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(Vertex, color) as u32,
            },
        ];

        let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo {
            vertex_attribute_description_count: vertex_input_attribute_descriptions.len() as u32,
            p_vertex_attribute_descriptions: vertex_input_attribute_descriptions.as_ptr(),
            vertex_binding_description_count: vertex_input_binding_descriptions.len() as u32,
            p_vertex_binding_descriptions: vertex_input_binding_descriptions.as_ptr(),
            ..Default::default()
        };

        let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };

        let surface_resolution = presenter.get_swapchain().get_resolution();

        let viewports = Self::get_viewports(surface_resolution);
        let scissors = Self::get_scissors(surface_resolution);

        let viewport_state_info = vk::PipelineViewportStateCreateInfo::builder()
            .scissors(&scissors)
            .viewports(&viewports);

        let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            line_width: 1.0,
            polygon_mode: vk::PolygonMode::FILL,
            ..Default::default()
        };
        let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };
        let noop_stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            ..Default::default()
        };

        let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
            depth_test_enable: 1,
            depth_write_enable: 1,
            depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
            front: noop_stencil_state,
            back: noop_stencil_state,
            max_depth_bounds: 1.0,
            ..Default::default()
        };
        let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::all(),
        }];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(&color_blend_attachment_states);

        let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state_info =
            vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_state);

        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stage_create_infos)
            .vertex_input_state(&vertex_input_state_info)
            .input_assembly_state(&vertex_input_assembly_state_info)
            .viewport_state(&viewport_state_info)
            .rasterization_state(&rasterization_info)
            .multisample_state(&multisample_state_info)
            .depth_stencil_state(&depth_state_info)
            .color_blend_state(&color_blend_state)
            .dynamic_state(&dynamic_state_info)
            .layout(layout)
            .render_pass(render_pass.get_vk_render_pass());

        let pipeline = unsafe {
            vk_device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[pipeline_info.build()],
                None,
            )
        }
        .expect("Can't create graphics pipeline.")
        .remove(0);

        unsafe {
            vk_device.destroy_shader_module(vert_shader, None);
            vk_device.destroy_shader_module(frag_shader, None);
        }
        Self {
            pipeline,
            layout,
            logger,
        }
    }

    pub fn get_vk_pipeline(&self) -> vk::Pipeline {
        self.pipeline
    }

    pub fn get_viewports(surface_resolution: vk::Extent2D) -> [vk::Viewport; 1] {
        [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: surface_resolution.width as f32,
            height: surface_resolution.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }]
    }

    pub fn get_scissors(surface_resolution: vk::Extent2D) -> [vk::Rect2D; 1] {
        [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: surface_resolution,
        }]
    }

    fn create_layout(vk_device: &ash::Device) -> vk::PipelineLayout {
        let create_info = vk::PipelineLayoutCreateInfo::default();
        unsafe { vk_device.create_pipeline_layout(&create_info, None) }
            .expect("Can't create pipeline")
    }

    fn load_shaders(vk_device: &ash::Device) -> (vk::ShaderModule, vk::ShaderModule) {
        let mut vert_spv_file = Cursor::new(&include_bytes!("../../shaders/triangle/vert.spv")[..]);
        let mut frag_spv_file = Cursor::new(&include_bytes!("../../shaders/triangle/frag.spv")[..]);
        let vert_shader_module = Self::create_shader_module(&mut vert_spv_file, vk_device);
        let frag_shader_module = Self::create_shader_module(&mut frag_spv_file, vk_device);
        (vert_shader_module, frag_shader_module)
    }

    fn create_shader_module(
        cursor: &mut Cursor<&[u8]>,
        vk_device: &ash::Device,
    ) -> vk::ShaderModule {
        let code = ash::util::read_spv(cursor).expect("Can't read spv file.");
        let shader_info = vk::ShaderModuleCreateInfo::builder().code(&code);
        unsafe { vk_device.create_shader_module(&shader_info, None) }
            .expect("Vertex shader module error")
    }

    pub fn destroy(&mut self, vk_device: &ash::Device) {
        debug!(self.logger, "Pipeline destroy() called.");
        unsafe {
            vk_device.destroy_pipeline(self.pipeline, None);
            debug!(self.logger, "\tPipeline destroyed");
            vk_device.destroy_pipeline_layout(self.layout, None);
            debug!(self.logger, "\tPipeline layout destroyed");
        }
    }
}
