
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
      0 => planeta_neon(fragment, uniforms),
      1 => planeta_raro(fragment, uniforms),
      2 => planeta_saturno(fragment, uniforms),
      3 => planeta_azul(fragment, uniforms),
      4 => planeta_celular(fragment, uniforms),
      5 => planeta_mancha(fragment, uniforms),
      6 => sol(fragment, uniforms),
      7 => planeta_rocoso(fragment, uniforms),
      8 => planeta_gaseoso(fragment, uniforms),
      9 => planeta_arcilla(fragment, uniforms),
      _ => planeta_mancha(fragment, uniforms),
  }
}



fn planeta_raro(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color_1 = Color::new(255, 0, 255); 
    let color_2 = Color::new(0, 255, 255); 
    let color_3 = Color::new(0, 255, 127);
    let color_4 = Color::new(255, 105, 180); 
    let color_5 = Color::new(255, 165, 0);  

    let position = fragment.vertex_position;

    let t = uniforms.time as f32 * 0.04; 
    let swirl = (position.x * 10.0 + position.y * 10.0 + t).sin(); 

    let noise_zoom = 7.0;
    let noise_value = uniforms.noise.get_noise_3d(
        position.x * noise_zoom,
        position.y * noise_zoom,
        position.z * noise_zoom + t,
    ).abs(); 

    let wave_value = (position.y * 12.0 + swirl * 5.0).sin();

    let threshold_1 = -0.6;
    let threshold_2 = -0.2;
    let threshold_3 = 0.2;
    let threshold_4 = 0.6;

    let base_color = if wave_value < threshold_1 {
        color_1.lerp(&color_2, noise_value)
    } else if wave_value < threshold_2 {
        color_2.lerp(&color_3, noise_value)
    } else if wave_value < threshold_3 {
        color_3.lerp(&color_4, noise_value)
    } else if wave_value < threshold_4 {
        color_4.lerp(&color_5, noise_value)
    } else {
        color_5.lerp(&color_1, noise_value)
    };

    base_color * fragment.intensity
}
  
fn planeta_saturno(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let color_1 = Color::new(255, 204, 102); 
  let color_2 = Color::new(255, 153, 51);  
  let color_3 = Color::new(204, 102, 0);  
  let color_4 = Color::new(153, 76, 0);   
  let color_5 = Color::new(102, 51, 0);  

  let position = fragment.vertex_position;

  let t = uniforms.time as f32 * 0.02; 
  let pulsate = (t * 0.5).sin() * 0.5; 

  let zoom = 10.0; 
  let bands_value = ((position.y * zoom) + pulsate).sin(); 

  let threshold_1 = -0.8;
  let threshold_2 = -0.4;
  let threshold_3 = 0.0;
  let threshold_4 = 0.4;

  let base_color = if bands_value < threshold_1 {
      color_1
  } else if bands_value < threshold_2 {
      color_2
  } else if bands_value < threshold_3 {
      color_3
  } else if bands_value < threshold_4 {
      color_4
  } else {
      color_5
  };

  base_color * fragment.intensity
}
  
fn planeta_azul(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color_1 = Color::new(173, 216, 230); 
    let color_2 = Color::new(135, 206, 250);
    let color_3 = Color::new(0, 191, 255); 
    let color_4 = Color::new(64, 224, 208); 
    let color_5 = Color::new(0, 206, 209);   
    let color_6 = Color::new(70, 130, 180); 
    let color_7 = Color::new(0, 105, 148); 
    let color_8 = Color::new(25, 25, 112);   

    let position = fragment.vertex_position;

    let t = uniforms.time as f32 * 0.02;
    let pulsate = (t * 0.5).sin() * 0.5; 

    let zoom = 15.0; 
    let bands_value = ((position.y * zoom) + pulsate).sin(); 

    let threshold_1 = -0.8;
    let threshold_2 = -0.6;
    let threshold_3 = -0.4;
    let threshold_4 = -0.2;
    let threshold_5 = 0.0;
    let threshold_6 = 0.2;
    let threshold_7 = 0.4;
    let threshold_8 = 0.6;

    // Asignar colores basados en el valor de las bandas
    let base_color = if bands_value < threshold_1 {
        color_1
    } else if bands_value < threshold_2 {
        color_2
    } else if bands_value < threshold_3 {
        color_3
    } else if bands_value < threshold_4 {
        color_4
    } else if bands_value < threshold_5 {
        color_5
    } else if bands_value < threshold_6 {
        color_6
    } else if bands_value < threshold_7 {
        color_7
    } else {
        color_8
    };

    base_color * fragment.intensity
}
  
fn planeta_celular(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let ring_color_1 = Color::new(85, 107, 47);   
  let ring_color_2 = Color::new(124, 252, 0);  
  let ring_color_3 = Color::new(34, 139, 34);   
  let ring_color_4 = Color::new(173, 255, 47);  

  let position = fragment.vertex_position;

  let t = uniforms.time as f32 * 0.03; 
  let pulsate = (t * 0.5).sin() * 0.2; 

  let zoom = 600.0; 
  let noise_value = uniforms.noise.get_noise_2d(
      (position.x + pulsate) * zoom, 
      position.z * zoom + t,         
  ).abs();

  let ring_threshold_1 = 0.1;
  let ring_threshold_2 = 0.3;
  let ring_threshold_3 = 0.5;
  let ring_threshold_4 = 0.7;

  let ring_color = if noise_value < ring_threshold_1 {
      ring_color_1
  } else if noise_value < ring_threshold_2 {
      ring_color_2
  } else if noise_value < ring_threshold_3 {
      ring_color_3
  } else {
      ring_color_4
  };

  ring_color * fragment.intensity
}

  
fn planeta_mancha(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let spot_color = Color::new(139, 69, 19);  
    let rock_base_color = Color::new(210, 105, 30); 
    let highlight_color = Color::new(255, 140, 0); 
    let dot_color = Color::new(255, 222, 173); 

    let position = fragment.vertex_position;

    let t = uniforms.time as f32 * 0.03;
    let pulsate = (t * 0.6).sin() * 0.5 + 0.5; 

    let rock_zoom = 15.0; 
    let rock_noise_value = uniforms.noise.get_noise_3d(
        position.x * rock_zoom,
        position.y * rock_zoom,
        position.z * rock_zoom,
    ).abs();

    let spot_zoom = 15.0; 
    let spot_noise_value = uniforms.noise.get_noise_2d(
        position.x * spot_zoom,
        position.y * spot_zoom,
    ).abs(); 

    let spot_threshold = 0.2 * pulsate; 

    let dots_zoom = 50.0;
    let dots_noise_value = uniforms.noise.get_noise_2d(
        position.x * dots_zoom,
        position.y * dots_zoom,
    ).abs(); 

    let dots_threshold = 0.05; 

    let base_color = if spot_noise_value < spot_threshold {
        spot_color.lerp(&rock_base_color, rock_noise_value)
    } else {
        rock_base_color.lerp(&highlight_color, rock_noise_value)
    };
 
    let final_color = if dots_noise_value < dots_threshold {
        dot_color  
    } else {
        base_color  
    };
 
    final_color * fragment.intensity
}


fn sol(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let core_color = Color::new(255, 255, 200);  
  let mid_color = Color::new(255, 223, 0);    
  let corona_color = Color::new(255, 140, 0);  
 
  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );
 
  let base_frequency = 0.5;  
  let pulsate_amplitude = 0.6;  
  let t = uniforms.time as f32 * 0.02;  
 
  let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

  let zoom = 1000.0;  
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
  let noise_value = (noise_value1 + noise_value2) * 0.5;  
 
  let blended_color = core_color
      .lerp(&mid_color, noise_value.abs())
      .lerp(&corona_color, (noise_value * 0.5 + 0.5).clamp(0.0, 1.0));
 
  blended_color * fragment.intensity
}

fn planeta_rocoso(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let color_1 = Color::new(245, 222, 179);  
  let color_2 = Color::new(222, 184, 135);  
  let color_3 = Color::new(210, 180, 140);  
  let color_4 = Color::new(188, 143, 143);  
  let color_5 = Color::new(205, 133, 63);   
  let color_6 = Color::new(139, 69, 19);   
  let color_7 = Color::new(160, 82, 45);   
 
  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );
 
  let t = uniforms.time as f32 * 0.01; 
  let pulsate = (t * 0.5).sin() * 0.1;  
 
  let zoom = 1000.0;  
  let noise_value1 = uniforms.noise.get_noise_3d(
      (position.x + pulsate) * zoom,
      (position.y + pulsate) * zoom,
      position.z * zoom + t,  
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x + 1000.0 + pulsate) * zoom,
      (position.y + 1000.0 + pulsate) * zoom,
      position.z * zoom + t, 
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5; 

  let stone_threshold_1 = -0.4;
  let stone_threshold_2 = -0.2;
  let stone_threshold_3 = 0.0;
  let stone_threshold_4 = 0.2;
  let stone_threshold_5 = 0.4;
  let stone_threshold_6 = 0.6;
 
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
 
  let light_dir = Vec3::new(1.0, 1.0, 0.5).normalize(); 
  let diffuse_intensity = dot(&light_dir, &fragment.normal).max(0.0);
 
  let final_color = base_color * (0.6 + 0.4 * diffuse_intensity);

  final_color * fragment.intensity
}


fn planeta_gaseoso(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let cloud_color = Color::new(255, 255, 255);  
  let fog_color = Color::new(120, 120, 120);   

  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );

  let t = uniforms.time as f32 * 0.01; 
  let pulsate = (t * 0.3).sin() * 0.5; 

  let zoom = 200.0; 
  let noise_value1 = uniforms.noise.get_noise_3d(
      (position.x + pulsate) * zoom,
      (position.y + pulsate) * zoom,
      position.z * zoom + t, 
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x - pulsate) * zoom,
      (position.y - pulsate) * zoom,
      position.z * zoom - t, 
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5; 

  let gradient = (1.0 - position.y.abs()).clamp(0.0, 1.0); 

  let final_color = cloud_color
      .lerp(&fog_color, noise_value.abs())
      .lerp(&fog_color, 1.0 - gradient);

  final_color * fragment.intensity
}


fn planeta_arcilla(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let color_1 = Color::new(173, 216, 230); 
  let color_2 = Color::new(135, 206, 250);
  let color_3 = Color::new(70, 130, 180);  
  let color_4 = Color::new(30, 144, 255);  
  let color_5 = Color::new(0, 105, 148);   

  let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth,
  );

  let t = uniforms.time as f32 * 0.02; 
  let pulsate = (t * 0.3).sin() * 0.3; 

  let zoom = 500.0; 
  let noise_value1 = uniforms.noise.get_noise_3d(
      (position.x + pulsate) * zoom,
      (position.y + pulsate) * zoom,
      position.z * zoom + t, 
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x - pulsate) * zoom,
      (position.y - pulsate) * zoom,
      position.z * zoom - t, 
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5; 

  let gradient = (1.0 - position.y.abs()).clamp(0.0, 1.0); 

  let threshold_1 = -0.2;
  let threshold_2 = 0.0;
  let threshold_3 = 0.2;
  let threshold_4 = 0.4;

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

  let final_color = base_color
      .lerp(&color_5, 1.0 - gradient) 
      * fragment.intensity;

  final_color
}

fn planeta_neon(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let color_1 = Color::new(255, 20, 147);  
  let color_2 = Color::new(0, 191, 255);   
  let color_3 = Color::new(50, 205, 50);   
  let color_4 = Color::new(255, 255, 0);   
  let color_5 = Color::new(75, 0, 130);    

  let position = fragment.vertex_position;

  let t = uniforms.time as f32 * 0.04; 
  let wave_movement = (position.x * 10.0 + position.y * 10.0 + t).sin(); 

  let zoom = 10.0; 
  let wave_value = ((position.x * zoom) + wave_movement).sin(); 

  let threshold_1 = -0.8;
  let threshold_2 = -0.4;
  let threshold_3 = 0.0;
  let threshold_4 = 0.4;

  let base_color = if wave_value < threshold_1 {
      color_1
  } else if wave_value < threshold_2 {
      color_2
  } else if wave_value < threshold_3 {
      color_3
  } else if wave_value < threshold_4 {
      color_4
  } else {
      color_5
  };

  base_color * fragment.intensity
}
