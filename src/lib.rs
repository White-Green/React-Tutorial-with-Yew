use wasm_bindgen::prelude::*;
use yew::prelude::*;
use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::Window;
use web_sys::HtmlElement;
use web_sys::HtmlAnchorElement;
use web_sys::Document;
use web_sys::Exception;

macro_rules! jsarray {
    ($($e:expr),*)=>{{
        let arr=js_sys::Array::new();
        $(
            arr.push(&wasm_bindgen::JsValue::from($e));
        )*
        arr
    }};
}

macro_rules! clog {
    ($($e:expr),*)=>{web_sys::console::log(&jsarray!($($e),*))}
}

#[derive(Clone, Copy, PartialEq)]
enum SquareState {
    None,
    X,
    O,
}

impl ToString for SquareState {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from(""),
            Self::X => String::from("X"),
            Self::O => String::from("O")
        }
    }
}

#[derive(Clone, Properties)]
struct SquareProperties {
    state: SquareState,
}

fn square(props: SquareProperties, callback: Callback<MouseEvent>) -> Html {
    html! {
            <button class="square" onclick=callback>
                {props.state}
            </button>
        }
}

struct Board {
    link: ComponentLink<Self>,
    props: BoardProperties,
}

enum BoardMsg {
    ClickHandle(usize)
}

#[derive(Clone, Properties)]
struct BoardProperties {
    squares: [SquareState; 9],
    x_is_next: bool,
    onclick: Callback<usize>,
}

impl Component for Board {
    type Message = BoardMsg;
    type Properties = BoardProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ClickHandle(i) => {
                self.props.onclick.emit(i);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        if self.props.squares != _props.squares || self.props.x_is_next != _props.x_is_next {
            self.props.squares = _props.squares;
            self.props.x_is_next = _props.x_is_next;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="board-row">
                      {self.render_square(0)}
                      {self.render_square(1)}
                      {self.render_square(2)}
                </div>
                <div class="board-row">
                      {self.render_square(3)}
                      {self.render_square(4)}
                      {self.render_square(5)}
                </div>
                <div class="board-row">
                      {self.render_square(6)}
                      {self.render_square(7)}
                      {self.render_square(8)}
                </div>
            </div>
        }
    }
}

impl Board {
    fn render_square(&self, i: usize) -> Html {
        html! {
            {square(SquareProperties{state:self.props.squares[i]},self.link.callback(move|_|{BoardMsg::ClickHandle(i)}))}
        }
    }
}

fn calculate_winner(squares: &[SquareState; 9]) -> SquareState {
    const LINES: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for [a, b, c] in LINES.iter() {
        let a = a.clone();
        let b = b.clone();
        let c = c.clone();
        if squares[a] != SquareState::None && squares[a] == squares[b] && squares[b] == squares[c] {
            return squares[a];
        }
    }
    SquareState::None
}

struct Game {
    link: ComponentLink<Self>,
    props: GameProperties,
}

enum GameMsg {
    ClickHandle(usize),
    JumpTo(usize),
}

#[derive(Clone, Properties)]
struct GameProperties {
    history: Vec<[SquareState; 9]>,
    step_number: usize,
    x_is_next: bool,
}

impl Component for Game {
    type Message = GameMsg;
    type Properties = GameProperties;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GameMsg::ClickHandle(i) => {
                let squares = self.props.history.get(self.props.step_number).unwrap_or(&[SquareState::None; 9]);
                if calculate_winner(squares) != SquareState::None || squares[i] != SquareState::None {
                    return false;
                }
                let mut squares = squares.clone();
                squares[i] = if self.props.x_is_next { SquareState::X } else { SquareState::O };
                self.props.history = Vec::from(&self.props.history[..=self.props.step_number]);
                self.props.history.push(squares);
                self.props.x_is_next = !self.props.x_is_next;
                self.props.step_number += 1;
                true
            }
            GameMsg::JumpTo(i) => {
                self.props.step_number = i;
                self.props.x_is_next = i % 2 == 0;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let status = match calculate_winner(&self.props.history.get(self.props.step_number).unwrap_or(&[SquareState::None; 9])) {
            SquareState::None => format!("Next player: {}", if self.props.x_is_next { "X" } else { "O" }),
            state => format!("Winner: {}", state.to_string()),
        };
        let squares = match self.props.history.get(self.props.step_number) {
            Some(sq) => sq.clone(),
            None => [SquareState::None; 9],
        };
        let list = self.props.history.iter().enumerate().map(|(i, _)| {
            let desc = if i > 0 {
                format!("Go to move #{}", i)
            } else {
                format!("Go to game start")
            };
            // let callback = self.link.callback(|_| GameMsg::JumpTo(i));
            html! {
                <li>
                    <button onclick=self.link.callback(move|_| GameMsg::JumpTo(i))>{desc}</button>
                </li>
            }
        });
        html! {
            <div class="game">
                <div class="game-board">
                    <Board squares=squares x_is_next=self.props.x_is_next onclick=self.link.callback(|i|GameMsg::ClickHandle(i))/>
                </div>
                <div class="game-info">
                    <div>{status}</div>
                    <ol>{for list}</ol>
                </div>
            </div>
        }
    }
}

//==================================================================================================

macro_rules! add_event_listener {
    ($doc:expr,$name:expr,$cls:expr)=>{{
        let closure = Closure::wrap(Box::new($cls) as Box<dyn FnMut(_)>);
        let result = $doc.add_event_listener_with_callback($name, closure.as_ref().unchecked_ref());
        closure.forget();
        result
    }}
}

fn init_event_listener(document: Document) -> Result<(), JsValue> {
    let doc = document.clone();
    let result = add_event_listener!(document, "mousedown", move |_: web_sys::MouseEvent| {
        if let Some(body) = doc.body() {
            body.class_list().add_1("mouse-navigation").unwrap_or_else(|e|clog!(e));
            body.class_list().remove_1("kbd-navigation").unwrap_or_else(|e|clog!(e));
        }
    });

    let doc = document.clone();
    let result = result.and(add_event_listener!(document, "keydown", move |e: web_sys::KeyboardEvent| {
        if e.key_code() == 9 {
            if let Some(body) = doc.body() {
                body.class_list().add_1("kbd-navigation").unwrap_or_else(|e|clog!(e));
                body.class_list().remove_1("mouse-navigation").unwrap_or_else(|e|clog!(e));
            }
        }
    }));

    let result = result.and(add_event_listener!(document, "click", move |e: web_sys::MouseEvent| {
        if let Some(target) = e.target() {
            if let Some(anchor) = target.dyn_ref::<HtmlAnchorElement>() {
                if anchor.href() == "#" {
                    e.prevent_default();
                }
            }
        }
    }));
    result
}

fn init_errors_view(window: &Window, document: Document) {
    if let Some(errors) = document.get_element_by_id("errors") {
        if let Some(errors) = errors.dyn_ref::<HtmlElement>() {
            let errors = errors.clone();
            let closure = move |message: String, source: String, lineno: i32, colno: i32, error: JsValue| {
                let text =
                    if error.is_null() || error.is_undefined() {
                        format!("{}(at {} : {} : {})", message, source, lineno, colno)
                    } else {
                        if let Some(exc) = error.dyn_ref::<Exception>() {
                            exc.stack()
                        } else {
                            if let Some(object) = error.as_ref().dyn_ref::<Object>() {
                                String::from(object.to_string())
                            } else {
                                "Object Cast Error".to_string()
                            }
                        }
                    };
                let text = format!("{}\n{}", errors.text_content().unwrap_or("".to_string()), text);
                errors.set_text_content(Some(&text));
                errors.style().remove_property("display").map_or_else(|_| (), |e| clog!(e));
            };
            let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut(_, _, _, _, _)>);
            window.set_onerror(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        } else {
            clog!("errors element is not HtmlElement.");
        }
    } else {
        clog!("errors element is not found.");
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    init_event_listener(document.clone())
        .unwrap_or_else(|e| {
            clog!("addEventListener failed", e);
        });

    init_errors_view(&window, document.clone());

    if let Some(entry) = document.get_element_by_id("app") {
        App::<Game>::new().mount_with_props(entry, GameProperties { history: vec![[SquareState::None; 9]], step_number: 0, x_is_next: true });
    } else {
        clog!("entry point element is not found.");
    }
}
