//demonstrates a fuzzy search bar 
use fltk::*;
use fltk::{app::*, window::*, misc::*};
use texture_synthesis as ts;
use indicatif::{ProgressBar};


#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

#[derive(Debug, Clone)]
struct MyProgress{
    total_pb: ProgressBar,
    stage_pb: ProgressBar,
    totalPerc: f64,
    total_len: usize,
    stage_len: usize,
    stage_num: u32,
    testButt: Progress,
    sender: fltk::app::Sender<Message>,
}


impl  ts::GeneratorProgress for MyProgress{
    fn update(&mut self, update: texture_synthesis::ProgressUpdate) {
        let scc = self.sender.clone();
        std::thread::spawn(move||{
            scc.send(Message::Test);
        });
        if update.total.total != self.total_len {
            self.total_len = update.total.total;
            self.total_pb.set_length(self.total_len as u64);
            self.testButt.redraw();
        }
        if update.stage.total != self.stage_len {
            self.stage_len = update.stage.total;
            self.stage_pb.set_length(self.stage_len as u64);
            self.stage_num += 1;
            self.stage_pb.set_message(&self.stage_num.to_string());
        }
        self.total_pb.set_position(update.total.current as u64);
        self.stage_pb.set_position(update.stage.current as u64);
        println!("totalcurrent/total is {}", (update.total.current as f32/update.total.total as f32).to_string());
        println!("total is: {} current total is: {}", update.total.total.to_string(), update.total.current.to_string());
        println!("total current is: {}", update.total.current.to_string());


        self.testButt.redraw();
        self.totalPerc = ((update.total.current as f32/update.total.total as f32) as f64)*100.0;
        
        self.testButt.set_value(((update.total.current as f32/update.total.total as f32) as f64)*100.0);
    }
}
fn main() {
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    

    let gen_prog = MyProgress{
        total_pb: ProgressBar::new(100),
        stage_pb: ProgressBar::new(100),
        stage_len: 0,
        stage_num: 0,
        total_len: 0,
        totalPerc: 0.0,
        testButt: Progress::new(100,100,100,50, ""),
        sender: s.clone(),
    };
    
    let mut pc = gen_prog.testButt.clone();
    let scc = s.clone();

    std::thread::spawn(move ||{
        generate(&mut pc, scc);
    });

    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    println!("{}", "got test message");
                    app::redraw();
                }
            }
        }
    }
}

fn generate(
    generation_prog_bar: &mut fltk::misc::Progress,
    s: fltk::app::Sender<Message>) -> Result<(), ts::Error>{
    let mut rng = rand::thread_rng();
    let randSeed = 1;
    let output_dim = ts::Dims::new(2000,2000);
    
    let gen_prog = MyProgress{
        total_pb: ProgressBar::new(100),
        stage_pb: ProgressBar::new(100),
        stage_len: 0,
        stage_num: 0,
        total_len: 0,
        totalPerc: 0.0,
        testButt: generation_prog_bar.clone(),
        sender: s.clone(),
    };
    println!("progres is {:?}", gen_prog);

    
    let progress = gen_prog;
    let outputPath = std::path::Path::new("tmp/previewImg.jpg");
    let mut sesh = ts::Session::builder()
        .add_example(&"imgs/smiley.png")
        .output_size(ts::Dims::square(500))
        .seed(randSeed)
        .build()?;

    let generated = sesh.run(Some(Box::new(progress)));
    generated.save(&outputPath)?;
    generated.save_debug("tmp/");
    Ok(())
}