use crate::Device;

pub struct TextRender {
    pub(crate) atlas: glyphon::TextAtlas,
    pub(crate) cache: glyphon::SwashCache,

}


impl TextRender {
    pub fn new(device: &Device) -> TextRender {
        let atlas = glyphon::TextAtlas::new(&device.device, &device.queue, &device.cache, device.texture_format);
        let cache = glyphon::SwashCache::new();
        TextRender {
            atlas,
            cache,
        }
    }
}