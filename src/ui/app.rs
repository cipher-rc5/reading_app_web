use crate::database::{Article, IndexedDbClient, Settings};
use eframe::egui;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen_futures::spawn_local;

enum AppMessage {
    Initialized {
        client: Arc<IndexedDbClient>,
        articles: Vec<Article>,
        settings: Option<Settings>,
    },
    InitializationFailed(String),
    ArticlesLoaded(Vec<Article>),
    ArticleSaved(Article),
    ArticleDeleted(String),
    SettingsSaved,
    DataError(String),
}

pub struct ReadingApp {
    db_client: Option<Arc<IndexedDbClient>>,
    current_article: Option<Article>,
    articles: Vec<Article>,
    settings: Settings,
    loading: bool,
    error_message: Option<String>,
    messages: Rc<RefCell<Vec<AppMessage>>>,

    show_settings: bool,
    show_editor: bool,
    editor_title: String,
    editor_content: String,
    editing_article_id: Option<String>,
}

impl Default for ReadingApp {
    fn default() -> Self {
        Self {
            db_client: None,
            current_article: None,
            articles: Vec::new(),
            settings: Settings {
                id: "default".to_string(),
                font_size: 16.0,
                font_family: "Default".to_string(),
                theme: "Light".to_string(),
            },
            loading: true,
            error_message: None,
            messages: Rc::new(RefCell::new(Vec::new())),
            show_settings: false,
            show_editor: false,
            editor_title: String::new(),
            editor_content: String::new(),
            editing_article_id: None,
        }
    }
}

impl ReadingApp {
    fn apply_settings(&self, ctx: &egui::Context) {
        let visuals = if self.settings.theme == "Dark" {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        let base_size = self.settings.font_size;

        if let Some(font_id) = style.text_styles.get_mut(&egui::TextStyle::Small) {
            font_id.size = (base_size - 2.0).max(10.0);
        }
        if let Some(font_id) = style.text_styles.get_mut(&egui::TextStyle::Body) {
            font_id.size = base_size;
        }
        if let Some(font_id) = style.text_styles.get_mut(&egui::TextStyle::Button) {
            font_id.size = base_size;
        }
        if let Some(font_id) = style.text_styles.get_mut(&egui::TextStyle::Monospace) {
            font_id.size = base_size;
        }
        if let Some(font_id) = style.text_styles.get_mut(&egui::TextStyle::Heading) {
            font_id.size = base_size + 8.0;
        }

        ctx.set_style(style);
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app = Self::default();

        let ctx = cc.egui_ctx.clone();
        let messages = Rc::clone(&app.messages);
        spawn_local(async move {
            match IndexedDbClient::new().await {
                Ok(client) => {
                    web_sys::console::log_1(&"IndexedDB initialized successfully".into());
                    let client: Arc<IndexedDbClient> = Arc::new(client);

                    let mut errors = Vec::new();

                    let articles = match client.get_all_articles().await {
                        Ok(articles) => {
                            web_sys::console::log_1(
                                &format!("Loaded {} articles", articles.len()).into(),
                            );
                            articles
                        }
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Failed to load articles: {}", e).into(),
                            );
                            errors.push(format!("Failed to load articles: {e}"));
                            Vec::new()
                        }
                    };

                    let settings = match client.get_settings().await {
                        Ok(settings) => settings,
                        Err(e) => {
                            web_sys::console::error_1(
                                &format!("Failed to load settings: {}", e).into(),
                            );
                            errors.push(format!("Failed to load settings: {e}"));
                            None
                        }
                    };

                    {
                        let mut queue = messages.borrow_mut();
                        queue.push(AppMessage::Initialized {
                            client,
                            articles,
                            settings,
                        });
                        for err in errors {
                            queue.push(AppMessage::DataError(err));
                        }
                    }

                    ctx.request_repaint();
                }
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Failed to initialize IndexedDB: {}", e).into(),
                    );
                    messages
                        .borrow_mut()
                        .push(AppMessage::InitializationFailed(e.to_string()));
                    ctx.request_repaint();
                }
            }
        });

        app
    }

    fn load_articles(&mut self) {
        if let Some(db) = &self.db_client {
            let db = Arc::clone(db);
            let messages = Rc::clone(&self.messages);
            spawn_local(async move {
                match db.get_all_articles().await {
                    Ok(articles) => {
                        web_sys::console::log_1(
                            &format!("Loaded {} articles", articles.len()).into(),
                        );
                        messages
                            .borrow_mut()
                            .push(AppMessage::ArticlesLoaded(articles));
                    }
                    Err(e) => {
                        web_sys::console::error_1(
                            &format!("Failed to load articles: {}", e).into(),
                        );
                        messages.borrow_mut().push(AppMessage::DataError(format!(
                            "Failed to load articles: {e}"
                        )));
                    }
                }
            });
        }
    }

    fn save_article(&mut self, article: Article) {
        if let Some(db) = &self.db_client {
            let db = Arc::clone(db);
            let messages = Rc::clone(&self.messages);
            spawn_local(async move {
                match db.save_article(&article).await {
                    Ok(_) => {
                        web_sys::console::log_1(&"Article saved successfully".into());
                        messages
                            .borrow_mut()
                            .push(AppMessage::ArticleSaved(article));
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to save article: {}", e).into());
                        messages.borrow_mut().push(AppMessage::DataError(format!(
                            "Failed to save article: {e}"
                        )));
                    }
                }
            });
        }
    }

    fn delete_article(&mut self, id: String) {
        if let Some(db) = &self.db_client {
            let db = Arc::clone(db);
            let messages = Rc::clone(&self.messages);
            spawn_local(async move {
                match db.delete_article(&id).await {
                    Ok(_) => {
                        web_sys::console::log_1(&"Article deleted successfully".into());
                        messages.borrow_mut().push(AppMessage::ArticleDeleted(id));
                    }
                    Err(e) => {
                        web_sys::console::error_1(
                            &format!("Failed to delete article: {}", e).into(),
                        );
                        messages.borrow_mut().push(AppMessage::DataError(format!(
                            "Failed to delete article: {e}"
                        )));
                    }
                }
            });
        }
    }

    fn process_messages(&mut self) {
        let queued = {
            let mut queue = self.messages.borrow_mut();
            queue.drain(..).collect::<Vec<_>>()
        };

        if queued.is_empty() {
            return;
        }

        for message in queued {
            match message {
                AppMessage::Initialized {
                    client,
                    articles,
                    settings,
                } => {
                    self.db_client = Some(client);
                    self.articles = articles;
                    if let Some(settings) = settings {
                        self.settings = settings;
                    }
                    self.loading = false;
                    self.error_message = None;

                    if !self.articles.is_empty() {
                        self.current_article =
                            self.articles.iter().max_by_key(|a| a.created_at).cloned();
                    }
                }
                AppMessage::InitializationFailed(error) => {
                    self.loading = false;
                    self.error_message = Some(format!("Initialization failed: {error}"));
                }
                AppMessage::ArticlesLoaded(articles) => {
                    self.articles = articles;
                    self.error_message = None;

                    if !self.articles.is_empty() {
                        self.current_article =
                            self.articles.iter().max_by_key(|a| a.created_at).cloned();
                    }
                }
                AppMessage::ArticleSaved(article) => {
                    if let Some(pos) = self
                        .articles
                        .iter()
                        .position(|existing| existing.id == article.id)
                    {
                        self.articles[pos] = article.clone();
                    } else {
                        self.articles.push(article.clone());
                    }
                    self.current_article = Some(article);
                    self.show_editor = false;
                    self.editor_title.clear();
                    self.editor_content.clear();
                    self.editing_article_id = None;
                    self.load_articles();
                }
                AppMessage::ArticleDeleted(id) => {
                    self.articles.retain(|a| a.id != id);
                    if let Some(current) = &self.current_article {
                        if current.id == id {
                            self.current_article = None;
                        }
                    }
                    self.error_message = None;
                }
                AppMessage::SettingsSaved => {
                    self.error_message = None;
                }
                AppMessage::DataError(error) => {
                    self.error_message = Some(error);
                }
            }
        }
    }

    fn calculate_word_count(text: &str) -> usize {
        text.split_whitespace().count()
    }

    fn calculate_reading_time(word_count: usize) -> u32 {
        ((word_count as f32) / 200.0).ceil() as u32
    }

    fn article_preview(content: &str, max_chars: usize) -> String {
        let mut preview = content.chars().take(max_chars).collect::<String>();
        if content.chars().count() > max_chars {
            preview.push_str("...");
        }
        preview
    }
}

impl eframe::App for ReadingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_messages();
        self.apply_settings(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Reading App (Web Edition)");

            if self.loading {
                ui.spinner();
                ui.label("Initializing IndexedDB...");
                return;
            }

            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("New Article").clicked() {
                    self.show_editor = true;
                    self.show_settings = false;
                    self.editor_title.clear();
                    self.editor_content.clear();
                    self.editing_article_id = None;
                }

                if ui.button("Settings").clicked() {
                    self.show_settings = !self.show_settings;
                    self.show_editor = false;
                }

                if ui.button("Article List").clicked() {
                    self.show_settings = false;
                    self.show_editor = false;
                    self.current_article = None;
                }

                ui.label(format!("Saved Articles: {}", self.articles.len()));
            });

            ui.separator();

            if self.show_editor {
                self.show_editor_panel(ui);
            } else if self.show_settings {
                self.show_settings_panel(ui);
            } else if let Some(article) = &self.current_article.clone() {
                self.show_article(ui, article);
            } else {
                self.show_article_list(ui);
            }
        });
    }
}

impl ReadingApp {
    fn show_editor_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Article Editor");

        ui.horizontal(|ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut self.editor_title);
        });

        ui.separator();

        ui.label("Content:");
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.editor_content)
                        .desired_width(f32::INFINITY)
                        .desired_rows(20),
                );
            });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Save Article").clicked() {
                if !self.editor_title.is_empty() && !self.editor_content.is_empty() {
                    let word_count = Self::calculate_word_count(&self.editor_content);
                    let reading_time = Self::calculate_reading_time(word_count);

                    let article = Article {
                        id: self
                            .editing_article_id
                            .clone()
                            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                        title: self.editor_title.clone(),
                        content: self.editor_content.clone(),
                        created_at: chrono::Utc::now().timestamp(),
                        word_count,
                        reading_time_minutes: reading_time,
                    };

                    self.save_article(article);
                }
            }

            if ui.button("Cancel").clicked() {
                self.show_editor = false;
                self.editor_title.clear();
                self.editor_content.clear();
                self.editing_article_id = None;
            }
        });
    }

    fn show_article(&mut self, ui: &mut egui::Ui, article: &Article) {
        ui.horizontal(|ui| {
            ui.heading(&article.title);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Delete").clicked() {
                    self.delete_article(article.id.clone());
                }
                if ui.button("Edit").clicked() {
                    self.show_editor = true;
                    self.editor_title = article.title.clone();
                    self.editor_content = article.content.clone();
                    self.editing_article_id = Some(article.id.clone());
                }
            });
        });

        ui.label(format!(
            "Word count: {} | Reading time: {} min",
            article.word_count, article.reading_time_minutes
        ));
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(&article.content);
        });
    }

    fn show_article_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Your Articles");

        if self.articles.is_empty() {
            ui.label("No articles yet. Create one to get started!");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let articles = self.articles.clone();
                for article in articles {
                    let frame = egui::Frame::group(ui.style()).inner_margin(egui::Margin::same(12));
                    frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading(&article.title);
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("Open").clicked() {
                                        self.current_article = Some(article.clone());
                                    }
                                },
                            );
                        });

                        ui.label(format!(
                            "{} words | {} min read",
                            article.word_count, article.reading_time_minutes
                        ));

                        ui.label(Self::article_preview(&article.content, 140));
                    });
                    ui.add_space(8.0);
                }
            });
        }
    }

    fn show_settings_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");

        ui.horizontal(|ui| {
            ui.label("Font Size:");
            ui.add(egui::Slider::new(&mut self.settings.font_size, 12.0..=24.0));
        });

        ui.horizontal(|ui| {
            ui.label("Theme:");
            egui::ComboBox::from_label("")
                .selected_text(&self.settings.theme)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.settings.theme, "Light".to_string(), "Light");
                    ui.selectable_value(&mut self.settings.theme, "Dark".to_string(), "Dark");
                });
        });

        if ui.button("Save Settings").clicked() {
            if let Some(db) = &self.db_client {
                let db = Arc::clone(db);
                let settings = self.settings.clone();
                let messages = Rc::clone(&self.messages);
                spawn_local(async move {
                    if let Err(e) = db.save_settings(&settings).await {
                        web_sys::console::error_1(
                            &format!("Failed to save settings: {}", e).into(),
                        );
                        messages.borrow_mut().push(AppMessage::DataError(format!(
                            "Failed to save settings: {e}"
                        )));
                    } else {
                        messages.borrow_mut().push(AppMessage::SettingsSaved);
                    }
                });
            }
        }
    }
}
