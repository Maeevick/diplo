use std::collections::HashMap;

use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use diplo_core::{Carnivore, Herbivore, Obstacle};

use crate::art;

/// Every picture, turned into a texture once at start-up.
#[derive(Resource)]
pub struct Pictures {
    herbivores: HashMap<Herbivore, Handle<Image>>,
    carnivores: HashMap<Carnivore, Handle<Image>>,
    glyphs: HashMap<char, Handle<Image>>,
    pub tree: Handle<Image>,
    pub rock: Handle<Image>,
    pub drumstick: Handle<Image>,
    pub arrow: Handle<Image>,
    pub check: Handle<Image>,
    pub grass: Handle<Image>,
    pub track: Handle<Image>,
    pub flowers: Handle<Image>,
}

impl Pictures {
    pub fn herbivore(&self, herbivore: Herbivore) -> Handle<Image> {
        self.herbivores[&herbivore].clone()
    }

    pub fn carnivore(&self, carnivore: Carnivore) -> Handle<Image> {
        self.carnivores[&carnivore].clone()
    }

    pub fn obstacle(&self, obstacle: Obstacle) -> Handle<Image> {
        match obstacle {
            Obstacle::Tree => self.tree.clone(),
            Obstacle::Rock => self.rock.clone(),
        }
    }

    pub fn glyph(&self, letter: char) -> Option<Handle<Image>> {
        self.glyphs.get(&letter).cloned()
    }
}

/// Builds the textures while the app is assembled: the first screen opens before any start-up system runs.
pub fn plugin(app: &mut App) {
    let pictures = load(&mut app.world_mut().resource_mut::<Assets<Image>>());
    app.insert_resource(pictures);
}

fn load(images: &mut Assets<Image>) -> Pictures {
    let mut texture = |picture: &[&str]| {
        let (width, height, pixels) = art::rgba(picture);
        let mut image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            pixels,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        image.sampler = ImageSampler::nearest();
        images.add(image)
    };
    let herbivores = HashMap::from([
        (Herbivore::Diplodocus, texture(art::DIPLODOCUS)),
        (Herbivore::Triceratops, texture(art::TRICERATOPS)),
        (Herbivore::Ankylosaurus, texture(art::ANKYLOSAURUS)),
    ]);
    let carnivores = HashMap::from([
        (Carnivore::Velociraptor, texture(art::VELOCIRAPTOR)),
        (Carnivore::Atrociraptor, texture(art::ATROCIRAPTOR)),
        (Carnivore::Spinosaurus, texture(art::SPINOSAURUS)),
        (Carnivore::TRex, texture(art::T_REX)),
        (Carnivore::Giganotosaurus, texture(art::GIGANOTOSAURUS)),
    ]);
    let glyphs = art::FONT
        .iter()
        .map(|(letter, rows)| (*letter, texture(rows)))
        .collect();
    Pictures {
        herbivores,
        carnivores,
        glyphs,
        tree: texture(art::TREE),
        rock: texture(art::ROCK),
        drumstick: texture(art::DRUMSTICK),
        arrow: texture(art::ARROW),
        check: texture(art::CHECK),
        grass: texture(art::GRASS),
        track: texture(art::TRACK),
        flowers: texture(art::FLOWERS),
    }
}
