fn main() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    }).unwrap()
}

mod m1 {
    use std::sync::mpsc::{channel, RecvError};
    use threadpool::ThreadPool;
    use num::complex::Complex;
    use image::{ImageBuffer, Pixel, Rgb};

    #[test]
    fn main() -> Result<(), ()> {
        let (width, height) = (1920, 1080);
        let mut img = ImageBuffer::new(width, height);
        let iterations = 300;

        let c = Complex::new(-0.8, 0.156);

        let pool = ThreadPool::new(num_cpus::get());
        let (tx, rx) = channel();

        for y in 0..height {
            let tx = tx.clone();
            pool.execute(move || for x in 0..width {
                let i = julia(c, x, y, width, height, iterations);
                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                tx.send((x, y, pixel)).expect("Could not send data!");
            });
        }

        for _ in 0..(width * height) {
            let (x, y, pixel) = rx.recv()?;
            img.put_pixel(x, y, pixel);
        }
        let _ = img.save("output.png")?;
        Ok(())
    }
}
