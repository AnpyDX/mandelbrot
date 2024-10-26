use std::{ self, env, fs };
use mandelbrot::cmds::parse_cmds;
use mandelbrot::renderer::{ RenderTarget, Renderer };

fn main() {
    // parse arguments
    let args = parse_cmds(env::args().skip(1).collect())
                                    .expect(
"error: failed to parse command arguments!
usage: mandelbrot <output> <width>x<height> <bounds-upperleft> <bounds-downright>
");
    
    // render image
    let output = fs::File::create(args.output.clone())
                            .expect("failed to create output file!");
    drop(output);

    let mut image_buffer = RenderTarget::new(args.size);

    println!("[info] rendering begins.");
    Renderer::render(args.size.1 as usize / 2, &mut image_buffer, &args.bounds);
    println!("[info] rendering finished.");

    println!("[info] writing target from buffer.");
    image_buffer.store(&args.output)
                        .expect("filed to write output file!");
    println!("[info] finished. Output: \"{}\".", args.output);
}