use fltk::{window::Window, prelude::{WidgetExt, GroupExt, WindowExt, WidgetBase}, button::Button, group::Flex, frame::Frame, enums::Event};


pub struct Message {
    win: Window,
    bt: Button,
}

impl Message {

    pub fn show(msg: &str) -> Self{

        let mut win = Window::default().with_label("Status Download!!!")
            .with_size(600, 130)
            .center_screen();

            let mut fx = Flex::default_fill().column();

                Frame::default();

                let msg = Frame::default().with_label(msg);
                fx.set_size(&msg, 60);

                Frame::default();

                let mut fx_bt = Flex::default(); 

                    Frame::default();

                    let bt = Button::default().with_label("Close");

                    fx_bt.set_size(&bt, 80);

                fx_bt.set_margin(3);

                fx_bt.end();
                fx.set_size(&fx_bt, 40);

            fx.end();

        win.end();
        win.show();


        let mut bt_ = bt.clone();
        win.handle(move |_,e|
        
            match e {
                Event::Hide => {

                    bt_.do_callback();
                    true
                },
                _=> {false}
            }
        );

        Self { win: win, bt: bt }

    }

    pub fn on_close<ONC: FnMut() + 'static>(&mut self, mut on_click: ONC) {

        let mut w = self.win.clone();

        self.bt.set_callback(move|_|{

            on_click();
            w.hide();

        });
    }
}
