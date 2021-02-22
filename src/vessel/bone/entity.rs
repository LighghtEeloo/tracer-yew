use crate::util::*;
use super::time::*;
use super::identity::*;
use super::tag_set::*;



// Entity Area

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entity {
    id: EntityId,
    pub time: TimeStamp,
    pub face: Face,
    pub bubble: Bubble,
    pub process: ProcessStatus,
    pub tags: TagSet,
}

impl Default for Entity {
    fn default() -> Self { 
        Entity::new()
    }
}


pub type Face = String;
pub type Bubble = String;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProcessStatus {
    Done,
    Marching,
    Pending,
    Planning,
    New,
}


impl IdentityProduct<EntityId> for Entity {
    fn with_id(id: EntityId) -> Self {
        Self {
            id,
            time: TimeStamp::now(),
            face: Face::new(),
            bubble: Bubble::new(),
            process: ProcessStatus::New,
            tags: TagSet::new()
        }
    }
    fn id(&self) -> EntityId {
        self.id.clone()
    }
}

impl Entity {
    pub fn face(&self) -> &Face {
        &self.face
    }
    pub fn set_face(&mut self, face: Face) {
        self.face = face;
    }
    pub fn bubble(&self) -> &Bubble {
        &self.bubble
    }
    pub fn set_bubble(&mut self, bubble: Bubble) {
        self.bubble = bubble;
    }
    pub fn process(&self) -> &ProcessStatus {
        &self.process
    }
    pub fn set_process(&mut self, process: ProcessStatus) {
        self.process = process;
    }
}


use ProcessStatus::*;
impl ProcessStatus {
    pub fn type_str(&self) -> String {
        match self {
            Done => "Done",
            Marching => "Marching",
            Pending => "Pending",
            Planning => "Planning",
            New => "New",
        }.to_string()
    }
    pub fn reflect(name: &str) -> Self {
        match name {
            "Done" => Done,
            "Marching" => Marching,
            "Pending" => Pending,
            "Planning" => Planning,
            "New" => New,
            _ => New,
        }
    }
    pub fn vec_all() -> Vec<Self> {
        vec! {
            New,
            Planning,
            Pending,
            Marching,
            Done,
        }
    }
    pub fn type_src(&self) -> String {
        format!("static/icons/Process/{}.svg", Self::type_str(self))
    }
}