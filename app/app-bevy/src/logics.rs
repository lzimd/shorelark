use crate::*;
use helper::Bounds;
use simulation::{Animal, Food, Simulation};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(process_movements_system.system().label("stage1"))
            .add_system(
                process_collisions_system
                    .system()
                    .label("stage2")
                    .after("stage1"),
            )
            .add_system(
                process_brains_system
                    .system()
                    .label("stage3")
                    .after("stage2"),
            )
            .add_system(
                process_generation_system
                    .system()
                    .label("stage4")
                    .after("stage3"),
            );
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // simulation
    let bounds = Bounds::new(800., 600.);
    let simulation = Simulation::new();
    let world = simulation.world();
    // animals
    for animal in world.animals() {
        let mat4 = helper::transform_viewport_from_postion(bounds.to_vec2(), animal.position());
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                transform: Transform::from_matrix(mat4),
                sprite: Sprite::new(Vec2::new(10., 10.)),
                ..Default::default()
            })
            .insert(animal.clone());
    }

    // foods
    for food in world.foods() {
        let mat4 = helper::transform_viewport_from_postion(bounds.to_vec2(), food.position());
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.0, 1.0, 0.5).into()),
                transform: Transform::from_matrix(mat4),
                sprite: Sprite::new(Vec2::new(5., 5.)),
                ..Default::default()
            })
            .insert(food.clone());
    }

    commands.insert_resource(simulation);
    commands.insert_resource(bounds);
}

fn process_movements_system(
    time: Res<Time>,
    bounds: Res<Bounds>,
    mut animal_query: Query<(&mut Animal, &mut Transform)>,
) {
    for (mut animal, mut transform) in animal_query.iter_mut() {
        let vel = animal
            .rotation()
            .mul_vec3(Vec3::new(animal.speed(), 0., 0.));

        let mut position = animal.position() + vel * time.delta_seconds();
        position.x = helper::wrap(position.x, -0.5, 0.5);
        position.y = helper::wrap(position.y, -0.5, 0.5);
        animal.set_position(position);

        transform.translation = animal.position() * bounds.to_vec2().extend(1.0);
        transform.rotation = animal.rotation();
    }
}

fn process_collisions_system(
    bounds: Res<Bounds>,
    colli_query: QuerySet<(
        Query<(&mut Animal, &Transform, &Sprite)>,
        Query<(&mut Food, &mut Transform, &Sprite)>,
    )>,
) {
    let mut rng = rand::thread_rng();

    // Safety
    for (mut animal, animal_transform, sprite) in unsafe { colli_query.q0().iter_unsafe() } {
        let animal_size = sprite.size;

        // Safety
        for (mut food, mut food_transform, sprite) in unsafe { colli_query.q1().iter_unsafe() } {
            let collision = collide(
                animal_transform.translation,
                animal_size,
                food_transform.translation,
                sprite.size,
            );

            if let Some(_collision) = collision {
                animal.eat_food();
                food.re_random(&mut rng);
                food_transform.translation = (Transform::from_matrix(
                    helper::transform_viewport_from_postion(bounds.to_vec2(), food.position()),
                ))
                .translation;
            }
        }
    }
}

fn process_brains_system(
    mut animal_query: Query<(&mut Animal, &mut Transform, &Sprite)>,
    food_query: Query<(&Food, &Sprite)>,
) {
    let foods: Vec<Food> = food_query.iter().map(|(food, _)| food.clone()).collect();
    for (mut animal, _, _) in animal_query.iter_mut() {
        animal.process_brains(&foods);
    }
}

fn process_generation_system(
    bounds: Res<Bounds>,
    mut simulation: ResMut<Simulation>,
    mut animal_query: Query<&mut Animal>,
    mut food_query: Query<(&mut Food, &mut Transform)>,
) {
    if simulation.preprocess_generation() {
        let animals: Vec<Animal> = animal_query
            .iter_mut()
            .map(|animal| animal.clone())
            .collect();

        simulation.process_generation(&animals);
        println!("generation");

        // Reset World
        animal_query
            .iter_mut()
            .zip(simulation.world().animals())
            .for_each(|(mut entity, new_entity)| {
                entity.clone_from(new_entity);
            });

        let mut food_iter = simulation.world().foods().iter();

        for (mut food, mut transform) in food_query.iter_mut() {
            let new_food = food_iter.next();
            if let Some(new_food) = new_food {
                food.clone_from(new_food);
                transform.translation = (Transform::from_matrix(
                    helper::transform_viewport_from_postion(bounds.to_vec2(), food.position()),
                ))
                .translation;
            }
        }
    }
}
