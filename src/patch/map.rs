use std::fmt::Debug;
use std::slice::Iter;

use lopdf::{Document, ObjectId};

pub struct Mapper {
    map: Vec<Option<ObjectId>>,
    removed_stream_ids: Vec<ObjectId>,
}

impl Mapper {
    pub fn new(doc: &Document) -> Mapper {
        let mut map = vec![None];
        for i in doc.objects.keys() {
            map.push(Some(*i));
        }
         Mapper {map, removed_stream_ids: vec![]}
    }

    pub fn get_stream_ids(&self) -> &Vec<ObjectId> {
        &self.removed_stream_ids
    }

    pub fn iter(&mut self) -> Iter<'_, Option<(u32, u16)>> {
        self.map.iter()
    }

    pub fn len(&self) -> usize {
        self.map.len() - 1
    }

    pub fn index_of(&self, pos: ObjectId) -> Option<usize> {
        self.map.iter().position(|x| *x == Some(pos))
    }

    pub fn remap(&mut self, current_obj_id: (u32, u16), move_to: usize) -> usize {
        let index = self.index_of(current_obj_id);

        match index {
            Some(u) => {
                let id = self.map.remove(u);
                self.map.insert(move_to, id);
                u
            },
            None => panic!("Document doesn't contain the id {:?}", current_obj_id),
        }
    }


    pub fn free_id_at(&mut self, id_to_insert_after: u32) {
        self.map.insert(id_to_insert_after as usize, None);
    }

    pub fn free_ids_starting_from(&mut self, id_to_insert_after: u32, amount: u32) {
        for i in 1..=amount {
            self.free_id_at(id_to_insert_after + i);
        }
    }

    pub fn map_to_last(&mut self, current_obj_id: (u32, u16)) {
        let index = self.index_of(current_obj_id);

        match index {
            Some(u) => {
                let id = self.map.remove(u);
                self.map.push(id);
            },
            None => panic!("Document doesn't contain the id {:?}", current_obj_id),
        }
    }

    pub fn remove_id_at(&mut self, id_to_remove: &ObjectId, is_stream: bool) {
        let index = self.index_of(*id_to_remove);
        if let Some(u) = index {
            if is_stream {
                self.removed_stream_ids.push(self.map.get(u).unwrap().unwrap());
            }
            self.map.remove(u);
        }
    }
}

impl Debug for Mapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map: [")?;
        if f.alternate() {
            for i in 1..=self.len() {
                if let Some(x) = self.map[i] {
                    write!(f, "\n\t(\n\t\told: ({}, {}),\n\t\tnew: ({}, 0)\n\t),", x.0, x.1, i)?;
                }
            }
        } else {
            for i in 1..=self.len() {
                if let Some(x) = self.map[i] {
                    write!(f, "\n\t({}, {}) => ({}, 0)", x.0, x.1, i)?;
                }
            }
        }
        write!(f, "\n]")?;
        Ok(())
    }
}
