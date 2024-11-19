/*You need to develop a program to compute the coordinates of a point after several transformations. To
make it nice and clean, you chose to use homogeneous coordinates. O being the origin of both axis, here
are the transformations to be implemented:
• Translation,
• Scaling,
• Rotation centered at O,
• Reflection over any axis that passes through O,
• Any combination of the previous transformations.
USAGE
./102architect x y transfo1 arg1 [arg2] [transfo2 arg1 [arg2]] ...
DESCRIPTION
x abscissa of the original point
y ordinate of the original point
transfo arg1 [arg2]
-t i j translation along vector (i, j)
-z m n scaling by factors m (x-axis) and n (y-axis)
-r d rotation centered in O by a d degree angle
-s d reflection over the axis passing through O with an inclination
angle of d degrees */

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let x = args[1].parse::<i32>().unwrap();
    let y = args[2].parse::<i32>().unwrap();
    let mut pos = vec![vec![x as f64, y as f64, 1.0]];
    let mut arg1: i32;
    let mut arg2: i32;

    for mut i in 3..args.len() {
        if args[i] == "-t" {
            arg1 = args[i + 1].parse::<i32>().unwrap();
            arg2 = args[i + 2].parse::<i32>().unwrap();
            println!("Translation along vector ({:},{:})", arg1, arg2);
            i+=2;
            pos = translate(arg1 as f64, arg2 as f64, pos);
        }
        if args[i] == "-z" {
            arg1 = args[i + 1].parse::<i32>().unwrap();
            arg2 = args[i + 2].parse::<i32>().unwrap();
            println!("Scaling by factors {:} and {:}", arg1, arg2);
            i+=2;
            pos = scale(arg1 as f64, arg2 as f64, pos);
        }
        if args[i] == "-r" {
            arg1 = args[i + 1].parse::<i32>().unwrap();
            println!("Rotation by a {:} degrees angle", arg1);
            i+=1;
            pos = rotate(arg1 as f64, pos as Vec<Vec<f64>>);
        }
        if args[i] == "-s" {
            arg1 = args[i + 1].parse::<i32>().unwrap();
            println!("Reflection over an axis with an inclination angle of {:} degrees", arg1);
            i+=1;
            pos = reflect(arg1 as f64, pos as Vec<Vec<f64>>);
        }
    }
}
/*let matrice1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let matrice2 = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
    let matfin = mult_mat(matrice1, matrice2);
    println!("{:?}", matfin);
}*/

fn mult_mat(matrice1: Vec<Vec<f64>>, matrice2: Vec<Vec<f64>>)->Vec<Vec<f64>>
{
    let mut matfin:Vec<Vec<f64>> = vec![vec![0.0; 3], vec![0.0; 3], vec![0.0; 3]];

    for i in 0..matrice1.len() {
        for j in 0..matrice2[0].len() {
            for x in 0..matrice1[i].len() {
                matfin[i][j] += matrice1[i][x] * matrice2[x][j];
            }
        }
    }
    matfin
}
fn scale(x: f64, y: f64, pos: Vec<Vec<f64>>)-> Vec<Vec<f64>>
{
    let identite = vec![vec![x, 0.0, 0.0], vec![0.0, y, 0.0], vec![0.0, 0.0, 1.0]];
    let res_mat = mult_mat(identite, pos.clone());
    println!("{:?}", res_mat);
    res_mat
}
fn translate(x: f64, y: f64, pos: Vec<Vec<f64>>)-> Vec<Vec<f64>>
{
    let identite = vec![vec![1.0, 0.0, x], vec![0.0, 1.0, y], vec![0.0, 0.0, 1.0]];
    let res_mat = mult_mat(identite, pos.clone());
    println!("{:?}", res_mat);
    res_mat
}
fn rotate(x: f64, pos: Vec<Vec<f64>>)-> Vec<Vec<f64>>
{
    let identite = vec![vec![x.cos(), -x.sin(), 0.0], vec![x.sin(), x.cos(), 0.0], vec![0.0, 0.0, 1.0]];
    let res_mat = mult_mat(identite, pos);
    println!("{:?}", res_mat);
    res_mat
}
fn reflect(x: f64, pos: Vec<Vec<f64>>)-> Vec<Vec<f64>>
{
    let identite = vec![vec![x.cos(), x.sin(), 0.0], vec![x.sin(), -x.cos(), 0.0], vec![0.0, 0.0, 1.0]];
    let res_mat = mult_mat(identite, pos);
    println!("{:?}", res_mat);
    res_mat
}