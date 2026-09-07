use crate::core::input::InputState;
use crate::gfx::{GpuContext, pipeline::PipelineBuilder};
use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

pub struct State {
    pub window: Arc<Window>,
    pub gpu: GpuContext,
    pub input: InputState,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let gpu = GpuContext::new(window.clone()).await?;
        let input = InputState::new();

        let render_pipeline = PipelineBuilder::build_basic_pipeline(&gpu.device, gpu.config.format);
        Ok(Self {
            window,
            gpu,
            input,
            render_pipeline,
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.gpu.resize(new_size);
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> anyhow::Result<()> {
        let Some(output) = self.acquire_frame()? else {
            return Ok(());
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Primary Command Encoder"),
            });

        self.record_render_pass(&mut encoder, &view);

        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        self.gpu.queue.present(output);

        Ok(())
    }

    fn acquire_frame(&mut self) -> anyhow::Result<Option<wgpu::SurfaceTexture>> {
        match self.gpu.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture)
            | wgpu::CurrentSurfaceTexture::Suboptimal(texture) => Ok(Some(texture)),

            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.gpu.resize(self.window.inner_size());
                Ok(None)
            }

            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => Ok(None),
        }
    }

    /// Records a render pass that clears the screen to a specific color.
    fn record_render_pass(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })] as &[Option<wgpu::RenderPassColorAttachment>],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        });
        // TODO: Add render pipelines, bind groups, and draw calls for the simulation.

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1);
    }
}
