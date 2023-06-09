use opencv::{core, highgui, imgproc, prelude::*, videoio, Result};

fn main() -> Result<()> {
    let window = "video capture";
    highgui::named_window(window, 1)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;

    if !opened {
        panic!("Unable to open default camera!");
    }

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            let mut gray_frame = Mat::default();
            let mut hist_equalized = Mat::default();
            let mut blurred_frame = Mat::default();
            let mut out_frame = Mat::default();

            // convert to grayscale
            imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;

            // equalize histogram
            imgproc::equalize_hist(&gray_frame, &mut hist_equalized)?;

            // unsharp masking
            imgproc::gaussian_blur(
                &hist_equalized,
                &mut blurred_frame,
                core::Size::new(0, 0),
                3.0,
                0.0,
                core::BORDER_DEFAULT,
            )?;

            core::add_weighted(
                &hist_equalized,
                1.5,
                &blurred_frame,
                -0.5,
                0.0,
                &mut out_frame,
                -1,
            )?;

            // put text
            imgproc::put_text(
                &mut out_frame,
                "Hello, world",
                core::Point::new(10, 50),
                imgproc::FONT_HERSHEY_SIMPLEX,
                1.0,
                core::Scalar::new(255.0, 255.0, 255.0, 0.0),
                2,
                imgproc::LINE_8,
                false,
            )?;

            highgui::imshow(window, &out_frame)?;
        }
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    
    return Ok(());
}
