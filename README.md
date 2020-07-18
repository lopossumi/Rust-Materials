# Learning Rust Part 5: Materials

> 📚 Series: [Part 1](https://github.com/lopossumi/Rust-Hello) - [Part 2](https://github.com/lopossumi/Rust-Output-Image) - [Part 3](https://github.com/lopossumi/Rust-Vectors) - [Part 4](https://github.com/lopossumi/Rust-Rays) - [Part 5](https://github.com/lopossumi/Rust-Materials)

In the [previous session](https://github.com/lopossumi/Rust-Rays) we created some rays and colored two spheres with normal vectors. Now let's add some materials to our spheres.

## Random number generation and anti-aliasing

First, we need a random number generator. There is no such thing in the standard library, so let's import the ```rand``` crate. It comes with a [comprehensive set of documentation](https://rust-random.github.io/book/guide-start.html), where in the very first example it is stated that:

> The first thing you may have noticed is that we imported everything from the prelude. This is the lazy way to use rand, and like the standard library's prelude, only imports the most common items. If you don't wish to use the prelude, remember to import the Rng trait!

Lazy way sounds great! After adding a random number generator and moving the camera to its own file, our main program averages a bunch of rays for a single pixel:
```rust
// main.rs
...
    let mut rng = thread_rng();
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let mut color: Color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..SAMPLES_PER_PIXEL {
            let rand_x: f64 = rng.gen(); // value range 0.0..1.0
            let rand_y: f64 = rng.gen();
            let u = (rand_x + x as f64) / (IMAGE_WIDTH - 1) as f64;
            let v = 1.0 - (rand_y + y as f64) / (IMAGE_HEIGHT - 1) as f64;
            let ray = camera.get_ray(u, v);
            color = color + ray.color(&world);
        }
        color = color / SAMPLES_PER_PIXEL as f64;
        *pixel = Rgb(color.to_rgb());
    }
```
With 5 samples the edges look really jagged:

![two_spheres_5_samples.png](two_spheres_5_samples.png)

With 50 samples everything is a lot smoother (while taking 10 times as long):

![two_spheres_50_samples.png](two_spheres_50_samples.png)

So how long is that, and what can we do about it?

## Benchmarking

There are a few benchmarking tools for Rust, but let's start with a simple timer and a print statement to the error output:
```rust
    let start = std::time::Instant::now();
    // Do heavy stuff
    eprintln!("Elapsed: {:?}", start.elapsed());
```
```
PS D:\RustProjects\materials> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\rays.exe`
Elapsed: 9.5592981s
Done.
```
So with 50 samples the rendering takes 9.6 seconds on Ryzen 3400G. Not great, not terrible.

But wait, we are running it in debug mode. How about if we switch to release mode?
```
PS D:\RustProjects\materials> cargo run --release
    Finished release [optimized] target(s) in 0.09s
     Running `target\release\rays.exe`
Elapsed: 246.4353ms
Done.
```
So we get almost a 40x speedup for free, just by remembering to use the right compiler flag! With 50 samples per pixel and FullHD resolution it takes 6.2 seconds to render two balls on a single core.
