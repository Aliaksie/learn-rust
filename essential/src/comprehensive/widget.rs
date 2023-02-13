
const PADDING: usize = 8;

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error>;

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        let rs = self.draw_into(&mut buffer);
        
        match rs {
            Ok(()) => println!("{}", &buffer),
            Err(error) => println!("Ops. Please chec the error {:?}", error)
        };
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Label {

    fn width(&self) -> usize {
        self.label
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        writeln!(buffer, "{}", self.label)?;

        Ok(())
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + PADDING
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        let width = self.width();
        let mut label = String::new();
        self.label.draw_into(&mut label)?;

        writeln!(buffer, "+{:-<width$}+", "")?; // top line
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", &line)?;
        }
        writeln!(buffer, "+{:-<width$}+", "")?; // bottom line

        Ok(())
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        std::cmp::max(
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
            self.title.chars().count(),
        )
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        let mut inner = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner)?;
        }

        let window_width = self.width();

        writeln!(buffer, "+-{:-<window_width$}-+", "")?;
        writeln!(buffer, "| {:^window_width$} |", &self.title)?;
        writeln!(buffer, "+={:=<window_width$}=+", "")?;
       
        for line in inner.lines() {
            writeln!(buffer, "| {:window_width$} |", line)?;
        }
        writeln!(buffer, "+-{:-<window_width$}-+", "")?;

        Ok(())
    }
}

// fn main() {
//     let mut window = Window::new("Rust GUI Demo 1.23");
//     window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
//     window.add_widget(Box::new(Button::new(
//         "Click me!",
//         Box::new(|| println!("You clicked the button!")),
//     )));
//     window.draw();
// }
