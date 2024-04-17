
extern crate noise;

use noise::{NoiseFn, Perlin};


use std::f32::consts::PI;
use std::env;
use std::process;

use ggez::conf;
use ggez::graphics;
use ggez::event;
use ggez::input::keyboard;
use ggez::nalgebra::Matrix3;
use ggez::nalgebra::Matrix4;
use ggez::nalgebra::Point3;
use ggez::nalgebra::Point4;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::nalgebra as na;
use ggez::GameResult;
use ggez::Context;

const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH: f32 = 800.0;


struct Triangle{
    a: na::Point3<f32>,
    b: na::Point3<f32>,
    c: na::Point3<f32>,

}

struct MyMesh {
    triangles: Vec<Triangle>,
}


impl MyMesh {
    fn new() -> Self {
        MyMesh { triangles: Vec::new() }
    }
}

fn multiply_matrix_proj(vec: na::Point3<f32>, matrix: Matrix4<f32>, flip: bool)->na::Point3<f32>{
    let vec_4d: Point4<f32> = na::Point4::new(vec.x, vec.y, vec.z, 1.0);
    let mut result = matrix * vec_4d;
    let flip_matrix = na::Matrix4::new(
        -1.0, 0.0, 0.0, 0.0,
        0.0, -1.0, 0.0, 0.0,
        0.0, 0.0, -1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    if flip {result = flip_matrix * result}
    let w = result.w;
    if w != 0.0{
        result.x /= w;
        result.y /= w;
        result.z /= w;
    }

    na::Point3::new(result.x, result.y, result.z)
}
fn point_list(tri: Triangle)->[na::Point<f32, na::U2>; 3]{
    [na::Point2::new(tri.a.x,tri.a.y),na::Point2::new(tri.b.x,tri.b.y),na::Point2::new(tri.c.x,tri.c.y)]
}

fn scale_screen(p_list: [na::Point<f32, na::U2>; 3],screen_w: f32, screen_h: f32)->[na::Point<f32, na::U2>; 3]{
   [na::Point2::new((p_list[0].x + 1.0) *0.5 * screen_w, screen_h -(p_list[0].y +1.0)*0.5*screen_h),
        na::Point2::new((p_list[1].x + 1.0) *0.5 * screen_w, screen_h -(p_list[1].y +1.0)*0.5*screen_h),
        na::Point2::new((p_list[2].x + 1.0) *0.5 * screen_w, screen_h - (p_list[2].y +1.0)*0.5*screen_h)]
}


fn triangle_matrix(tri:&Triangle)->Matrix3<f32>{
    na::Matrix3::new(tri.a.x,tri.b.x,tri.c.x,
        tri.a.y,tri.b.y,tri.c.y,
        tri.a.z,tri.b.z,tri.c.z)
}
fn matrix_triangle(matrix: Matrix3<f32>)->Triangle{
    Triangle { a: Point3::new(matrix.m11, matrix.m21, matrix.m31), b:Point3::new(matrix.m12, matrix.m22, matrix.m32), c: Point3::new(matrix.m13, matrix.m23, matrix.m33) }

}

fn x_rotation(tri: Matrix3<f32>,  theta: f32)->Matrix3<f32>{
    let x_matrix = na::Matrix3::new(1.0, 0.0, 0.0,
    0.0, theta.cos(), theta.sin() * -1.0,
0.0 ,theta.sin(), theta.cos() );
     x_matrix*tri

}
fn y_rotation(tri: Matrix3<f32>, theta: f32)->Matrix3<f32>{
    let y_matrix = na::Matrix3::new(theta.cos(), 0.0, theta.sin(),
    0.0, 1.0, 0.0,
 - theta.sin() ,0.0, theta.cos() );
    y_matrix*tri

}
fn z_rotation(tri: Matrix3<f32>, theta :f32)->Matrix3<f32>{
    let z_matrix = na::Matrix3::new(theta.cos(),-1.0 * theta.sin(), 0.0,
        theta.sin(), theta.cos(), 0.0,
    0.0 ,0.0, 1.0 );
         z_matrix*tri 
}

fn new_cube() -> MyMesh {
    // Define the points
    let point_wbs = na::Point3::new(0.0, 0.0, 0.0);
    let point_ebs = na::Point3::new(1.0, 0.0, 0.0);
    let point_wbn = na::Point3::new(0.0, 0.0, 1.0);
    let point_ebn = na::Point3::new(1.0, 0.0, 1.0);

    let point_wus = na::Point3::new(0.0, 1.0, 0.0);
    let point_eus = na::Point3::new(1.0, 1.0, 0.0);
    let point_wun = na::Point3::new(0.0, 1.0, 1.0);
    let point_eun = na::Point3::new(1.0, 1.0, 1.0);

    let mut my_mesh = MyMesh::new();

    // Define the triangles for each side

    // South side
    my_mesh.triangles.push(Triangle { a: point_wbs, b: point_wus, c: point_eus });
    my_mesh.triangles.push(Triangle { a: point_wbs, b: point_eus, c: point_ebs });

    // East side
    my_mesh.triangles.push(Triangle { a: point_ebs, b: point_eus, c: point_eun });
    my_mesh.triangles.push(Triangle { a: point_ebs, b: point_eun, c: point_ebn });

    // North side
    my_mesh.triangles.push(Triangle { a: point_ebn, b: point_eun, c: point_wun });
    my_mesh.triangles.push(Triangle { a: point_ebn, b: point_wun, c: point_wbn });

    // West side
    my_mesh.triangles.push(Triangle { a: point_wbn, b: point_wun, c: point_wus });
    my_mesh.triangles.push(Triangle { a: point_wbn, b: point_wus, c: point_wbs });

    // Top side
    my_mesh.triangles.push(Triangle { a: point_wus, b: point_wun, c: point_eun });
    my_mesh.triangles.push(Triangle { a: point_wus, b: point_eun, c: point_eus });

    // Bottom side
    my_mesh.triangles.push(Triangle { a: point_ebn, b: point_wbn, c: point_wbs });
    my_mesh.triangles.push(Triangle { a: point_ebn, b: point_wbs, c: point_ebs });

    my_mesh
}

fn create_icosahedron() -> MyMesh {
    let phi = (1.0 + 5.0f32.sqrt()) / 2.0; // golden ratio
    let a = 1.0 / phi.sqrt();
    let b = phi * a;

    // The 12 vertices of an icosahedron
    let vertices = [
        na::Point3::new(-a,  b,  0.0),
        na::Point3::new( a,  b,  0.0),
        na::Point3::new(-a, -b,  0.0),
        na::Point3::new( a, -b,  0.0),
        na::Point3::new( 0.0, -a,  b),
        na::Point3::new( 0.0,  a,  b),
        na::Point3::new( 0.0, -a, -b),
        na::Point3::new( 0.0,  a, -b),
        na::Point3::new( b,  0.0, -a),
        na::Point3::new( b,  0.0,  a),
        na::Point3::new(-b,  0.0, -a),
        na::Point3::new(-b,  0.0,  a),
    ];

    let faces = [
        (0, 11, 5), (0, 5, 1), (0, 1, 7), (0, 7, 10), (0, 10, 11),
        (1, 5, 9), (5, 11, 4), (11, 10, 2), (10, 7, 6), (7, 1, 8),
        (3, 9, 4), (3, 4, 2), (3, 2, 6), (3, 6, 8), (3, 8, 9),
        (4, 9, 5), (2, 4, 11), (6, 2, 10), (8, 6, 7), (9, 8, 1),
    ];

    let mut triangles = Vec::new();
    for &(i, j, k) in &faces {
        triangles.push(Triangle {
            a: vertices[i],
            b: vertices[j],
            c: vertices[k],
        });
    }

    // Normalize vertices to make sure they are on the sphere's surface
    for tri in &mut triangles {
        tri.a = (tri.a.coords.normalize() * 1.0).into();
        tri.b = (tri.b.coords.normalize() * 1.0).into();
        tri.c = (tri.c.coords.normalize() * 1.0).into();
    }

    let mut sphere = MyMesh::new();
    sphere.triangles =triangles;
    sphere
}

fn generate_sphere_triangles(subdivisions: u32) -> MyMesh {
    let mut ico = create_icosahedron();
    

    for _ in 0..subdivisions {
        let mut new_triangles = Vec::new();
        for tri in ico.triangles {
            let ab_mid = na::Vector3::from((tri.a.coords + tri.b.coords) * 0.5).normalize();
            let bc_mid = na::Vector3::from((tri.b.coords + tri.c.coords) * 0.5).normalize();
            let ca_mid = na::Vector3::from((tri.c.coords + tri.a.coords) * 0.5).normalize();

            new_triangles.push(Triangle { a: tri.a, b: ab_mid.into(), c: ca_mid.into()});
            new_triangles.push(Triangle { a: ab_mid.into(), b: tri.b, c: bc_mid.into() });
            new_triangles.push(Triangle { a: ca_mid.into(), b: bc_mid.into(), c: tri.c });
            new_triangles.push(Triangle { a: ab_mid.into(), b: bc_mid.into(), c: ca_mid.into() });
        }
        ico.triangles = new_triangles;
    }

    // Normalize the vertices to make sure they're on the sphere surface
    for tri in &mut ico.triangles {
        tri.a = (tri.a.coords.normalize() ).into(); // radius is 1.0 for diameter of 2
        tri.b = (tri.b.coords.normalize() ).into();
        tri.c = (tri.c.coords.normalize() ).into();
    }

    ico
}

//can be used to plot something else than mountain
fn function_three_dim(x:f32, y:f32, _low: f32, __high: f32)->f32{
   x.powf(2.0) + y.powf(2.0)
    // 1.0
    //(10.0 * x).sin() *   y.cos()
}

fn moutain_slope(x:f32, z:f32, center: f32, height_coefficent:f32)->f32{
    1.0 + (1.0/(height_coefficent + (x - center).powf(2.0)+ (z-center).powf(2.0)))
}


fn mountain_function_perlin(x: f32, z: f32, low: f32, high: f32, height_coefficent:f32) -> f32 {
    let perlin = Perlin::new();
    let center = (high-low / 2.0 )+ low;
    let scale = 10.0;
    let elevation_scale = 0.1 * moutain_slope(x, z, center, height_coefficent);
    elevation_scale * perlin.get([x as f64 * scale, z as f64 * scale]) as f32
}

fn generate_function_triangles(resolution: usize, low:f32, high: f32, height_func: fn(f32, f32, f32,f32, f32)->f32, height_coefficient:f32) -> MyMesh {
    let mut triangles =  Vec::new();
    let width = high- low;

    for i in 0..resolution {
        for j in 0..resolution {
            // Create points (i, j), (i+1, j), (i, j+1), (i+1, j+1)
            let x0 = ((i as f32*width) - low )/ resolution as f32;
            let z0 = ((j as f32 * width) - low) / resolution as f32;
            let x1 = ((i + 1) as f32 * width - low) / resolution as f32;
            let z1 = ((j + 1) as f32 * width  - low)/ resolution as f32;

            let y00 = height_func(x0, z0, low, high, height_coefficient);
            let y10 = height_func(x1, z0, low, high, height_coefficient);
            let y01 = height_func(x0, z1, low, high, height_coefficient);
            let y11 = height_func(x1, z1, low, high, height_coefficient);

            // Create triangles for the square grid cell
            let tri1 = Triangle {
                a: na::Point3::new(x0, y01, z1),
                b: na::Point3::new(x0, y00, z0),
                c: na::Point3::new(x1, y11, z1),
            };

            let tri2 = Triangle {
                a: na::Point3::new(x1, y11, z1),
                b: na::Point3::new(x0, y00, z0),
                c: na::Point3::new(x1, y10, z0),
            };

            // Add the triangles to the mesh
            triangles.push(tri1);
            triangles.push(tri2);
        }
    }

    let plot = MyMesh{triangles};
    plot
    
}

//Generate a pair of testing triangles
fn test_triangle()->MyMesh{
    let triangle_1 = Triangle {
        a:na::Point3::new(0.0,0.0,1.0),
        b:na::Point3::new(0.0,0.5,0.0),
        c:na::Point3::new(1.0,0.5,1.0),
    };
    let triangle_2= Triangle{
        a:na::Point3::new(1.0,0.5,1.0),
        b:na::Point3::new(0.0,0.5,0.0),
        c:na::Point3::new(1.0,0.0,0.0),
    };

    let triangles: Vec<Triangle> = vec![triangle_1,triangle_2];
    MyMesh{triangles}

}

struct MainState{
    object: MyMesh,
    reverse_lighting: bool,
    theta_x: f32,
    theta_y: f32,
    theta_z: f32,
    move_x: f32,
    move_y: f32,
    move_z: f32,

}

impl MainState {
    pub fn new(_ctx: &mut Context, object_str: &str, res: u32, height_coefficient: f32)->Self{
        let object = match object_str {
            "cube" => new_cube(),
            "sphere" => generate_sphere_triangles(res),
            "mountain"=>generate_function_triangles(res as usize, 0.0, 1.0, mountain_function_perlin, height_coefficient),
            _=>new_cube()

        };
        let reverse_lighting = match object_str{
            "mountain"=>true,
            _=>false
        };

        MainState{
            object: object,
            reverse_lighting: reverse_lighting,
            theta_x: 0.0,
            theta_y: 0.0,
            theta_z: 0.0,
            move_x: 0.0,
            move_y: 0.0,
            move_z: 13.0,

        }
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context)->ggez::GameResult{
        
        
        if keyboard::is_key_pressed(ctx, event::KeyCode::Q){
            self.theta_x+=0.01;
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::E){
            self.theta_x-=0.01
        }

        if keyboard::is_key_pressed(ctx, event::KeyCode::Left){
            self.theta_y+=0.01;
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::Right){
            self.theta_y-=0.01
        }

        if keyboard::is_key_pressed(ctx, event::KeyCode::Up){
            self.theta_z+=0.01;
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::Down){
            self.theta_z-=0.01
        }


        if keyboard::is_key_pressed(ctx, event::KeyCode::S){
            self.move_z+=0.1;
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::W){
            self.move_z-=0.1
        }
        if keyboard::is_key_pressed(ctx, event::KeyCode::D){
            self.move_x+=0.1;
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::A){
            self.move_x-=0.1
        }
        if keyboard::is_mod_active(ctx, event::KeyMods::SHIFT){
            self.move_y+=0.1;
        } else if keyboard::is_mod_active(ctx, event::KeyMods::CTRL){
            self.move_y-=0.1
        }

        Ok(())
    } 
    
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        graphics::clear(ctx, graphics::BLACK);
        let mut draw_param = DrawParam::default();


        let f_near = 0.1;
        let f_far = 1000.0;
        let f_fov = 90.0;
        let f_aspect_ratio = screen_h/screen_w;
        let f_fov_rad = 1.0 / (f_fov * 0.5 / 180.0 * PI).tan();

        let projection_matrix = na::Matrix4::new(
        f_aspect_ratio*f_fov_rad, 0.0, 0.0, 0.0,
         0.0, f_fov_rad, 0.0, 0.0,
        0.0, 0.0, f_far/ (f_far - f_near), 1.0,
       0.0, 0.0, (-f_far * f_near)/(f_far - f_near), 0.0);

        //Init camera position
       let v_camera = na::Vector3::new(0.0,0.0,0.0);
        

        for tris in &self.object.triangles {
            let rot_matrix = triangle_matrix(tris);
            let z_rotated = z_rotation(rot_matrix, self.theta_z);
            let zx_rotated = x_rotation(z_rotated, self.theta_x);
            let zxy_rotated = y_rotation(zx_rotated, self.theta_y);

            let zxy_rotated_triangle = matrix_triangle(zxy_rotated);
            
            //Translate triangle
            //offset for view

            let mut tri_trans = zxy_rotated_triangle;

            tri_trans.a.x += self.move_x;
            tri_trans.b.x += self.move_x;
            tri_trans.c.x += self.move_x;
            
            tri_trans.a.y += self.move_y;
            tri_trans.b.y += self.move_y;
            tri_trans.c.y += self.move_y;
            
            tri_trans.a.z += self.move_z;
            tri_trans.b.z += self.move_z;
            tri_trans.c.z += self.move_z;

            // Calculate Normal
            
            let line1 = na::Vector3::new(
                tri_trans.b.x-tri_trans.a.x,
                 tri_trans.b.y-tri_trans.a.y,
                  tri_trans.b.z-tri_trans.a.z);
            let line2 = na::Vector3::new(
                    tri_trans.c.x-tri_trans.a.x,
                     tri_trans.c.y-tri_trans.a.y,
                      tri_trans.c.z-tri_trans.a.z);
                
            let mut normal = na::Vector3::new(
                (line1.y * line2.z) - (line1.z * line2.y),
                (line1.z * line2.x) - (line1.x * line2.z),
                (line1.x * line2.y) - (line1.y * line2.x) );

            //Makes the shapes back face lit up
            let reverse_lighting = self.reverse_lighting;

            let normal_vec = normal.dot(&(tri_trans.a.coords - v_camera));
            if normal_vec <=0.0  
                || reverse_lighting{

                if reverse_lighting && normal_vec > 0.0 {
                    normal *= -1.0;
                }
                //lighting
                let ambient_intensity = 0.05; // Ambient light intensity
                let diffuse_intensity = 1.4; // Diffuse light intensity
                let specular_intensity = 0.5; // Specular light intensity
                let shininess = 0.5; // Shininess factor for specular highlight


                let light_direction = na::Vector3::new(1.0,1.0,-1.0).normalize();
                let ambient = ambient_intensity;
                let mut diffuse =normal.dot(&light_direction) * diffuse_intensity;
                diffuse = diffuse.max(0.0).min(1.0);

                //specular component (Blinn-Phong)
                let view_direction = (v_camera - tri_trans.a.coords).normalize();
                let halfway_dir = (light_direction + view_direction).normalize();
                let mut specular = normal.dot(&halfway_dir).powf(shininess) * specular_intensity;
                specular = specular.max(0.0).min(1.0);

                
                let lum = ambient + diffuse + specular;
                // let lum = 1.0;
                let color = Color{r: lum, g: lum, b: lum, a:1.0};
            
            let flip =true;
                                                                                        
            let tri_proj =  Triangle{

                a: multiply_matrix_proj(tri_trans.a, projection_matrix, flip),
                b: multiply_matrix_proj(tri_trans.b, projection_matrix, flip),
                c: multiply_matrix_proj(tri_trans.c, projection_matrix, flip),
            
            };

            let scaled_points = scale_screen(point_list(tri_proj), screen_w, screen_h);
            
            let tri_mesh = Mesh::from_triangles(ctx, &scaled_points,  color)?;
            draw_param.dest = na::Point2::new(0.0,0.0).into();
            graphics::draw(ctx, &tri_mesh, draw_param)?;
            
            }
        
        }
        graphics::present(ctx)?;
        Ok(())
        }
        
    }
    
    
fn main() -> GameResult {
    let args: Vec<String> = env::args().collect();
    let help ="3 different commands, for cube: cube \nFor sphere: sphere [1-3:number of subdivisions of icosahedron]\nfor mountain: mountain [1-50: number of subdivisions of triangles (more is laggier)] [0.0-10.0: height coefficient, smaller is higher mountains near center]\n Q/E for x rotation, Left/Right for y rotation and Up/Down for z rotation. WASD and Shift/Ctrl to move object around";
    if args.len() < 2 {
        eprintln!("{}", help);
        process::exit(1);
    }

    let object = args[1].parse::<String>().unwrap_or_else(|err| {
        eprintln!("Error parsing object: {}", err);
        process::exit(1);
    });
    if object == "help"{
        eprintln!("{}", help);
        process::exit(1);
    }
    let mut res =1;
    if object!="cube"{
        res = args[2].parse::<u32>().unwrap_or_else(|err|{
            eprintln!("Error parsing resolution: {}", err);
            process::exit(1);
        });
    }
    let mut coeff= 1.0;
    if object=="mountain"{
        coeff = args[3].parse::<f32>().unwrap_or_else(|err|{
            eprintln!("Error parsing height coefficient: {}", err);
            process::exit(1);
        });
    }

    let cb =ggez::ContextBuilder::new("mountain_render","nico")
    .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (ctx, event_loop) = &mut cb.build()?;
    
    graphics::set_window_title(ctx, "mountain_render");

    let mut state = MainState::new(ctx, object.as_str(), res, coeff );
    event::run(ctx, event_loop,&mut state)?;
    Ok(())
}

