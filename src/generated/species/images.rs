/*
use super::species::Species;
use crate::generated::assets::loaded::Images;
use rand::seq::SliceRandom;

pub fn get_random_image(specie : Species) -> Images {
    let mut rng = rand::thread_rng();
    *match specie {
        Species::Human => vec![Images::GeneratedHumanBarbarianIdle01,Images::GeneratedHumanBarbarianIdle02,Images::GeneratedHumanBarbarianIdle03,Images::GeneratedHumanBarbarianIdle04],
        Species::Merfolk => vec![Images::GeneratedMerfolkLizardIdle01,Images::GeneratedMerfolkLizardIdle02,Images::GeneratedMerfolkLizardIdle03,Images::GeneratedMerfolkLizardIdle04]
    }.choose(&mut rng).unwrap()
}
*/
