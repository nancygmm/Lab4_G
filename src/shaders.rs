
use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, current_shader: u8) -> Color {
  match current_shader {
      1 => black_and_white(fragment, uniforms),
      2 => dalmata_shader(fragment, uniforms),
      3 => cloud_shader(fragment, uniforms),
      4 => cellular_shader(fragment, uniforms),
      5 => lava_shader(fragment, uniforms),
      6 => solar_shader(fragment, uniforms),
      7 => rock_shader(fragment, uniforms),
      8 => rainforest_shader(fragment, uniforms),
      9 => clay_shader(fragment, uniforms),
      _ => lava_shader(fragment, uniforms), // Shader por defecto si se selecciona un número no válido
  }
}



fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
  
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
  
    let random_number = rng.gen_range(0..=100);
  
    let black_or_white = if random_number < 50 {
      Color::new(0, 0, 0)
    } else {
      Color::new(255, 255, 255)
    };
  
    black_or_white * fragment.intensity
}
  
fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    let noise_value = uniforms.noise.get_noise_2d(
      (x + ox) * zoom,
      (y + oy) * zoom,
    );
  
    let spot_threshold = 0.5;
    let spot_color = Color::new(255, 255, 255); // White
    let base_color = Color::new(0, 0, 0); // Black
  
    let noise_color = if noise_value < spot_threshold {
      spot_color
    } else {
      base_color
    };
  
    noise_color * fragment.intensity
}
  
fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;  // to move our values 
    let ox = 100.0; // offset x in the noise map
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;
  
    let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);
  
    // Define cloud threshold and colors
    let cloud_threshold = 0.5; // Adjust this value to change cloud density
    let cloud_color = Color::new(255, 255, 255); // White for clouds
    let sky_color = Color::new(30, 97, 145); // Sky blue
  
    // Determine if the pixel is part of a cloud or sky
    let noise_color = if noise_value > cloud_threshold {
      cloud_color
    } else {
      sky_color
    };
  
    noise_color * fragment.intensity
}
  
fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0;  // Zoom factor to adjust the scale of the cell pattern
    let ox = 50.0;    // Offset x in the noise map
    let oy = 50.0;    // Offset y in the noise map
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    // Use a cellular noise function to create the plant cell pattern
    let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();
  
    // Define different shades of green for the plant cells
    let cell_color_1 = Color::new(85, 107, 47);   // Dark olive green
    let cell_color_2 = Color::new(124, 252, 0);   // Light green
    let cell_color_3 = Color::new(34, 139, 34);   // Forest green
    let cell_color_4 = Color::new(173, 255, 47);  // Yellow green
  
    // Use the noise value to assign a different color to each cell
    let final_color = if cell_noise_value < 0.15 {
      cell_color_1
    } else if cell_noise_value < 0.7 {
      cell_color_2
    } else if cell_noise_value < 0.75 {
      cell_color_3
    } else {
      cell_color_4
    };
  
    // Adjust intensity to simulate lighting effects (optional)
    final_color * fragment.intensity
}
  
fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base colors for the lava effect
    let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
    let dark_color = Color::new(130, 20, 0);   // Darker red-orange
  
    // Get fragment position
    let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth
    );
  
    // Base frequency and amplitude for the pulsating effect
    let base_frequency = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.01;
  
    // Pulsate on the z-axis to change spot size
    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;
  
    // Apply noise to coordinates with subtle pulsating on z-axis
    let zoom = 1000.0; // Constant zoom factor
    let noise_value1 = uniforms.noise.get_noise_3d(
      position.x * zoom,
      position.y * zoom,
      (position.z + pulsate) * zoom
    );
    let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x + 1000.0) * zoom,
      (position.y + 1000.0) * zoom,
      (position.z + 1000.0 + pulsate) * zoom
    );
    let noise_value = (noise_value1 + noise_value2) * 0.5;  // Averaging noise for smoother transitions
  
    // Use lerp for color blending based on noise value
    let color = dark_color.lerp(&bright_color, noise_value);
  
    color * fragment.intensity
}


fn solar_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para el efecto solar
  let core_color = Color::new(255, 255, 200); // Amarillo muy claro (casi blanco)
  let mid_color = Color::new(255, 223, 0);   // Amarillo dorado (más cercano al núcleo)
  let corona_color = Color::new(255, 140, 0); // Naranja suave para la corona externa

  // Obtener la posición del fragmento
  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );

  // Frecuencia y amplitud base para el efecto de pulsación
  let base_frequency = 0.5; // Frecuencia ajustada para un movimiento más dinámico
  let pulsate_amplitude = 0.6; // Amplitud ajustada para un efecto de movimiento más notable
  let t = uniforms.time as f32 * 0.02; // Velocidad de la animación incrementada

  // Efecto de pulsación para variar el ruido a lo largo del tiempo
  let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

  // Aplicar ruido a las coordenadas con una pulsación más visible
  let zoom = 1000.0; // Conservamos el zoom del diseño original para mantener el detalle fino
  let noise_value1 = uniforms.noise.get_noise_3d(
      position.x * zoom,
      position.y * zoom,
      (position.z + pulsate) * zoom,
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x + 1000.0) * zoom,
      (position.y + 1000.0) * zoom,
      (position.z + 1000.0 + pulsate) * zoom,
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5;  // Promediar el ruido para transiciones suaves

  // Interpolación de colores: del centro brillante al borde naranja suave
  let blended_color = core_color
      .lerp(&mid_color, noise_value.abs())
      .lerp(&corona_color, (noise_value * 0.5 + 0.5).clamp(0.0, 1.0));

  // Ajustar la intensidad para simular efectos de iluminación
  blended_color * fragment.intensity
}

fn rock_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para la textura rocosa con tonalidades beige
  let color_1 = Color::new(245, 222, 179); // Beige muy claro (blanco arena)
  let color_2 = Color::new(222, 184, 135); // Beige claro (arena)
  let color_3 = Color::new(210, 180, 140); // Beige medio-claro (arena clara)
  let color_4 = Color::new(188, 143, 143); // Beige medio (rosado suave)
  let color_5 = Color::new(205, 133, 63);  // Beige medio-oscuro (tierra clara)
  let color_6 = Color::new(139, 69, 19);   // Marrón claro (madera)
  let color_7 = Color::new(160, 82, 45);   // Marrón rojizo (tierra más oscura)

  // Obtener la posición del fragmento
  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );

  // Ajuste del tiempo para el desplazamiento
  let t = uniforms.time as f32 * 0.01; // Controla la velocidad del movimiento
  let pulsate = (t * 0.5).sin() * 0.1; // Movimiento suave para simular el flujo

  // Ajuste de ruido para generar la textura rocosa con movimiento
  let zoom = 1000.0; // Aumentar el zoom para obtener más detalles y muchas piedras pequeñas
  let noise_value1 = uniforms.noise.get_noise_3d(
      (position.x + pulsate) * zoom,
      (position.y + pulsate) * zoom,
      position.z * zoom + t, // Desplazamiento en el tiempo para el movimiento
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x + 1000.0 + pulsate) * zoom,
      (position.y + 1000.0 + pulsate) * zoom,
      position.z * zoom + t, // Desplazamiento en el tiempo para el movimiento
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5;  // Promediar el ruido para transiciones suaves

  // Umbrales para definir las áreas de "piedras" y "grietas"
  let stone_threshold_1 = -0.4;
  let stone_threshold_2 = -0.2;
  let stone_threshold_3 = 0.0;
  let stone_threshold_4 = 0.2;
  let stone_threshold_5 = 0.4;
  let stone_threshold_6 = 0.6;

  // Determinación del color basado en el valor de ruido
  let base_color = if noise_value > stone_threshold_6 {
      color_1
  } else if noise_value > stone_threshold_5 {
      color_2
  } else if noise_value > stone_threshold_4 {
      color_3
  } else if noise_value > stone_threshold_3 {
      color_4
  } else if noise_value > stone_threshold_2 {
      color_5
  } else if noise_value > stone_threshold_1 {
      color_6
  } else {
      color_7
  };

  // Simulación de relieve usando la normal del fragmento y una dirección de luz
  let light_dir = Vec3::new(1.0, 1.0, 0.5).normalize(); // Dirección de la luz ajustada para mayor contraste
  let diffuse_intensity = dot(&light_dir, &fragment.normal).max(0.0);

  // Ajuste de color basado en la intensidad difusa para dar efecto de relieve
  let final_color = base_color * (0.6 + 0.4 * diffuse_intensity);

  final_color * fragment.intensity
}


fn rainforest_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para la textura de niebla o nubes densas
  let cloud_color = Color::new(255, 255, 255); // Blanco brillante para las áreas densas
  let fog_color = Color::new(120, 120, 120);   // Gris para las áreas más tenues

  // Obtener la posición del fragmento
  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );

  // Ajuste del tiempo para el desplazamiento
  let t = uniforms.time as f32 * 0.01; // Controla la velocidad del movimiento
  let pulsate = (t * 0.3).sin() * 0.5; // Movimiento suave y sutil para simular el flujo de la niebla

  // Ajuste de ruido para generar la textura de niebla con movimiento
  let zoom = 200.0; // Ajuste del zoom para una textura de nubes más detallada
  let noise_value1 = uniforms.noise.get_noise_3d(
      (position.x + pulsate) * zoom,
      (position.y + pulsate) * zoom,
      position.z * zoom + t, // Desplazamiento en el tiempo para el movimiento
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x - pulsate) * zoom,
      (position.y - pulsate) * zoom,
      position.z * zoom - t, // Desplazamiento en el tiempo para el movimiento
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5; // Promediar el ruido para un efecto más suave

  // Crear un gradiente para dar densidad a la textura de las nubes
  let gradient = (1.0 - position.y.abs()).clamp(0.0, 1.0); // Mayor densidad en el centro, desvaneciéndose hacia los bordes

  // Mezclar el color de la nube con el de la niebla usando el valor de ruido y el gradiente
  let final_color = cloud_color
      .lerp(&fog_color, noise_value.abs())
      .lerp(&fog_color, 1.0 - gradient);

  // Ajustar la intensidad para simular la transparencia de la niebla
  final_color * fragment.intensity
}


fn clay_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para la textura con tonalidades azules y celestes
  let color_1 = Color::new(173, 216, 230); // Celeste muy claro
  let color_2 = Color::new(135, 206, 250); // Azul cielo claro
  let color_3 = Color::new(70, 130, 180);  // Azul intermedio (azul acero)
  let color_4 = Color::new(30, 144, 255);  // Azul más intenso (azul denso)
  let color_5 = Color::new(0, 105, 148);   // Azul oscuro

  // Obtener la posición del fragmento
  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );

  // Ajuste del tiempo para el desplazamiento
  let t = uniforms.time as f32 * 0.02; // Controla la velocidad del movimiento
  let pulsate = (t * 0.3).sin() * 0.3; // Movimiento suave para simular el flujo de la textura

  // Ajuste de ruido para generar la textura con movimiento
  let zoom = 500.0; // Ajuste del zoom para un detalle más fino
  let noise_value1 = uniforms.noise.get_noise_3d(
      (position.x + pulsate) * zoom,
      (position.y + pulsate) * zoom,
      position.z * zoom + t, // Desplazamiento en el tiempo para el movimiento
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x - pulsate) * zoom,
      (position.y - pulsate) * zoom,
      position.z * zoom - t, // Desplazamiento en el tiempo para el movimiento
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5; // Promediar el ruido para un efecto más uniforme

  // Crear un gradiente para simular el desvanecimiento de la textura
  let gradient = (1.0 - position.y.abs()).clamp(0.0, 1.0); // Mayor densidad en el centro, desvaneciéndose hacia los bordes

  // Definir los umbrales para las tonalidades de azul
  let threshold_1 = -0.2;
  let threshold_2 = 0.0;
  let threshold_3 = 0.2;
  let threshold_4 = 0.4;

  // Asignar colores basados en el valor de ruido
  let base_color = if noise_value > threshold_4 {
      color_1
  } else if noise_value > threshold_3 {
      color_2
  } else if noise_value > threshold_2 {
      color_3
  } else if noise_value > threshold_1 {
      color_4
  } else {
      color_5
  };

  // Mezclar el color de la textura con el gradiente para simular el desvanecimiento
  let final_color = base_color
      .lerp(&color_5, 1.0 - gradient) // Desvanece hacia un azul más oscuro en los bordes
      * fragment.intensity;

  final_color
}
