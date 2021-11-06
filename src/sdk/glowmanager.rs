/// CGlowObjectManager is deprecated and should not be used.

#[derive(Debug)]
pub struct CGlowObjectManager {
    base: *mut usize,
}

impl CGlowObjectManager {}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug)]
pub struct CGlowObjectDefinition_t {
    pub m_hEntity: super::entity::CEntity,
    pub m_vGlowColor: cgmath::Vector3<f32>,
    pub m_flGlowAlpha: f32,

    _u: [u8; 4],
    pub flUnk: f32,
    pub m_flBloomAmount: f32,
    pub localplayeriszeropoint3: f32,

    pub m_bRenderWhenOccluded: bool,
    pub m_bRenderWhenUnoccluded: bool,
    pub m_bFullBloomRender: bool,
    _u2: [u8; 4],

    pub m_nFullBloomStencilTestValue: i32,
    pub iUnk: i32,
    pub m_nSplitScreenSlot: i32,
    pub m_nNextFreeSlot: i32,
}

impl CGlowObjectDefinition_t {
    pub fn set_glow_color(&mut self, color: cgmath::Vector3<f32>) {
        self.m_vGlowColor = color;
    }

    pub fn get_entity(&self) -> super::entity::CEntity {
        self.m_hEntity
    }

    pub fn is_empty(&self) -> bool {
        self.m_nNextFreeSlot != -2
    }
}
