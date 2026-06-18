use egui_wgpu::wgpu;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;

pub struct DepthTarget {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    size: [u32; 2],
}

impl DepthTarget {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let (texture, view) = create_depth_texture(device, width, height);
        Self {
            texture,
            view,
            size: [width, height],
        }
    }

    pub fn ensure_size(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if self.size == [width, height] {
            return;
        }
        let (texture, view) = create_depth_texture(device, width, height);
        self.texture = texture;
        self.view = view;
        self.size = [width, height];
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

fn create_depth_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("terrain depth texture"),
        size: wgpu::Extent3d {
            width: width.max(1),
            height: height.max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}
