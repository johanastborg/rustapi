use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use chrono::Utc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{
        CreatePaperPayload, PaginatedResponse, Paper, PaperQuery, StoreStats, UpdatePaperPayload,
    },
};

#[derive(Clone)]
pub struct PaperStore {
    papers: Arc<RwLock<HashMap<String, Paper>>>,
}

impl PaperStore {
    pub fn new() -> Self {
        Self {
            papers: Arc::new(RwLock::new(Self::initial_seeds())),
        }
    }

    fn initial_seeds() -> HashMap<String, Paper> {
        let seeds = vec![
            Paper {
                id: "1706.03762".to_string(),
                title: "Attention Is All You Need".to_string(),
                authors: vec![
                    "Ashish Vaswani".to_string(),
                    "Noam Shazeer".to_string(),
                    "Niki Parmar".to_string(),
                    "Jakob Uszkoreit".to_string(),
                    "Llion Jones".to_string(),
                    "Aidan N. Gomez".to_string(),
                    "Łukasz Kaiser".to_string(),
                    "Illia Polosukhin".to_string(),
                ],
                summary: "The dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism. We propose a new simple network architecture, the Transformer, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely.".to_string(),
                categories: vec!["cs.CL".to_string(), "cs.LG".to_string(), "cs.AI".to_string()],
                primary_category: "cs.CL".to_string(),
                published: "2017-06-12T17:57:34Z".to_string(),
                updated: Some("2023-08-02T01:54:19Z".to_string()),
                doi: Some("10.48550/arXiv.1706.03762".to_string()),
                pdf_url: "https://arxiv.org/pdf/1706.03762.pdf".to_string(),
                arxiv_url: "https://arxiv.org/abs/1706.03762".to_string(),
                comment: Some("15 pages, 5 figures, NeurIPS 2017".to_string()),
                journal_ref: Some("NeurIPS 2017".to_string()),
            },
            Paper {
                id: "1512.03385".to_string(),
                title: "Deep Residual Learning for Image Recognition".to_string(),
                authors: vec![
                    "Kaiming He".to_string(),
                    "Xiangyu Zhang".to_string(),
                    "Shaoqing Ren".to_string(),
                    "Jian Sun".to_string(),
                ],
                summary: "Deeper neural networks are more difficult to train. We present a residual learning framework to ease the training of networks that are substantially deeper than those used previously. We explicitly reformulate the layers as learning residual functions with reference to the layer inputs, instead of learning unreferenced functions.".to_string(),
                categories: vec!["cs.CV".to_string()],
                primary_category: "cs.CV".to_string(),
                published: "2015-12-10T05:58:39Z".to_string(),
                updated: None,
                doi: Some("10.1109/CVPR.2016.90".to_string()),
                pdf_url: "https://arxiv.org/pdf/1512.03385.pdf".to_string(),
                arxiv_url: "https://arxiv.org/abs/1512.03385".to_string(),
                comment: Some("CVPR 2016 Best Paper Award".to_string()),
                journal_ref: Some("IEEE CVPR 2016".to_string()),
            },
            Paper {
                id: "1810.04805".to_string(),
                title: "BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding".to_string(),
                authors: vec![
                    "Jacob Devlin".to_string(),
                    "Ming-Wei Chang".to_string(),
                    "Kenton Lee".to_string(),
                    "Kristina Toutanova".to_string(),
                ],
                summary: "We introduce a new language representation model called BERT, which stands for Bidirectional Encoder Representations from Transformers. Unlike recent language representation models, BERT is designed to pre-train deep bidirectional representations from unlabeled text by jointly conditioning on both left and right context in all layers.".to_string(),
                categories: vec!["cs.CL".to_string(), "cs.AI".to_string(), "cs.LG".to_string()],
                primary_category: "cs.CL".to_string(),
                published: "2018-10-11T17:35:46Z".to_string(),
                updated: Some("2019-05-24T18:14:02Z".to_string()),
                doi: Some("10.48550/arXiv.1810.04805".to_string()),
                pdf_url: "https://arxiv.org/pdf/1810.04805.pdf".to_string(),
                arxiv_url: "https://arxiv.org/abs/1810.04805".to_string(),
                comment: Some("NAACL-HLT 2019 Long Paper".to_string()),
                journal_ref: Some("NAACL-HLT 2019".to_string()),
            },
            Paper {
                id: "2005.14165".to_string(),
                title: "Language Models are Few-Shot Learners".to_string(),
                authors: vec![
                    "Tom B. Brown".to_string(),
                    "Benjamin Mann".to_string(),
                    "Nick Ryder".to_string(),
                    "Melanie Subbiah".to_string(),
                    "Jared Kaplan".to_string(),
                    "Prafulla Dhariwal".to_string(),
                    "Arvind Neelakantan".to_string(),
                    "Pranav Shyam".to_string(),
                    "Girish Sastry".to_string(),
                    "Amanda Askell".to_string(),
                ],
                summary: "Recent work has demonstrated substantial gains on many NLP tasks and benchmarks by pre-training on a large corpus of text followed by fine-tuning on a specific task. While typically task-agnostic in architecture, this method still requires task-specific fine-tuning datasets. Here we show that scaling up language models greatly improves task-agnostic, few-shot performance.".to_string(),
                categories: vec!["cs.CL".to_string(), "cs.AI".to_string(), "cs.LG".to_string()],
                primary_category: "cs.CL".to_string(),
                published: "2020-05-28T18:00:00Z".to_string(),
                updated: Some("2020-07-22T19:00:00Z".to_string()),
                doi: Some("10.48550/arXiv.2005.14165".to_string()),
                pdf_url: "https://arxiv.org/pdf/2005.14165.pdf".to_string(),
                arxiv_url: "https://arxiv.org/abs/2005.14165".to_string(),
                comment: Some("75 pages, NeurIPS 2020".to_string()),
                journal_ref: Some("NeurIPS 2020".to_string()),
            },
            Paper {
                id: "2106.09685".to_string(),
                title: "LoRA: Low-Rank Adaptation of Large Language Models".to_string(),
                authors: vec![
                    "Edward J. Hu".to_string(),
                    "Yelong Shen".to_string(),
                    "Phillip Wallis".to_string(),
                    "Zeyuan Allen-Zhu".to_string(),
                    "Yuanzhi Li".to_string(),
                    "Shean Wang".to_string(),
                    "Lu Wang".to_string(),
                    "Weizhu Chen".to_string(),
                ],
                summary: "An important paradigm of natural language processing centers on large-scale pre-training on general domain data and adaptation to particular tasks or domains. As we pre-train larger models, full fine-tuning becomes increasingly costly. We propose Low-Rank Adaptation, or LoRA, which freezes the pre-trained model weights and injects trainable rank decomposition matrices into each layer of the Transformer architecture.".to_string(),
                categories: vec!["cs.CL".to_string(), "cs.AI".to_string(), "cs.LG".to_string()],
                primary_category: "cs.CL".to_string(),
                published: "2021-06-17T18:00:00Z".to_string(),
                updated: Some("2021-10-16T18:00:00Z".to_string()),
                doi: Some("10.48550/arXiv.2106.09685".to_string()),
                pdf_url: "https://arxiv.org/pdf/2106.09685.pdf".to_string(),
                arxiv_url: "https://arxiv.org/abs/2106.09685".to_string(),
                comment: Some("ICLR 2022".to_string()),
                journal_ref: Some("ICLR 2022".to_string()),
            },
            Paper {
                id: "2305.18290".to_string(),
                title: "Direct Preference Optimization: Your Language Model is Secretly a Reward Model".to_string(),
                authors: vec![
                    "Rafael Rafailov".to_string(),
                    "Archit Sharma".to_string(),
                    "Eric Mitchell".to_string(),
                    "Stefano Ermon".to_string(),
                    "Christopher D. Manning".to_string(),
                    "Chelsea Finn".to_string(),
                ],
                summary: "While large-scale unsupervised language models learn broad world knowledge and reasoning skills, achieving precise control over their behavior is difficult due to the completely unsupervised nature of their training. Existing techniques for gaining this steerability use Reinforcement Learning from Human Feedback (RLHF). We propose Direct Preference Optimization (DPO), which solves the constrained RLHF problem implicitly with a simple cross-entropy loss.".to_string(),
                categories: vec!["cs.LG".to_string(), "cs.AI".to_string(), "cs.CL".to_string()],
                primary_category: "cs.LG".to_string(),
                published: "2023-05-29T17:53:08Z".to_string(),
                updated: Some("2023-12-13T18:24:19Z".to_string()),
                doi: Some("10.48550/arXiv.2305.18290".to_string()),
                pdf_url: "https://arxiv.org/pdf/2305.18290.pdf".to_string(),
                arxiv_url: "https://arxiv.org/abs/2305.18290".to_string(),
                comment: Some("NeurIPS 2023 Oral".to_string()),
                journal_ref: Some("NeurIPS 2023".to_string()),
            },
            Paper {
                id: "2205.14135".to_string(),
                title: "FlashAttention: Fast and Memory-Efficient Exact Attention with IO-Awareness".to_string(),
                authors: vec![
                    "Tri Dao".to_string(),
                    "Daniel Y. Fu".to_string(),
                    "Stefano Ermon".to_string(),
                    "Atri Rudra".to_string(),
                    "Christopher Ré".to_string(),
                ],
                summary: "Transformers are slow and memory-hungry on long sequences, since the time and memory complexity of self-attention are quadratic in sequence length. We propose FlashAttention, an IO-aware exact attention algorithm that uses tiling to reduce the number of memory reads/writes between GPU high bandwidth memory (HBM) and GPU on-chip SRAM.".to_string(),
                categories: vec!["cs.LG".to_string(), "cs.AI".to_string()],
                primary_category: "cs.LG".to_string(),
                published: "2022-05-27T18:00:00Z".to_string(),
                updated: Some("2022-06-23T18:00:00Z".to_string()),
                doi: Some("10.48550/arXiv.2205.14135".to_string()),
                pdf_url: "https://arxiv.org/pdf/2205.14135.pdf".to_string(),
                arxiv_url: "https://arxiv.org/abs/2205.14135".to_string(),
                comment: Some("NeurIPS 2022".to_string()),
                journal_ref: Some("NeurIPS 2022".to_string()),
            },
        ];

        let mut map = HashMap::new();
        for paper in seeds {
            map.insert(paper.id.clone(), paper);
        }
        map
    }

    pub async fn list(&self, query: &PaperQuery) -> PaginatedResponse<Paper> {
        let papers_guard = self.papers.read().await;
        let mut list: Vec<Paper> = papers_guard.values().cloned().collect();

        // 1. Text Search Filter (q)
        if let Some(ref q) = query.q {
            let q_lower = q.to_lowercase();
            list.retain(|p| {
                p.id.to_lowercase().contains(&q_lower)
                    || p.title.to_lowercase().contains(&q_lower)
                    || p.summary.to_lowercase().contains(&q_lower)
                    || p.authors.iter().any(|a| a.to_lowercase().contains(&q_lower))
                    || p.categories.iter().any(|c| c.to_lowercase().contains(&q_lower))
            });
        }

        // 2. Category Filter
        if let Some(ref cat) = query.category {
            let cat_lower = cat.to_lowercase();
            list.retain(|p| {
                p.primary_category.to_lowercase() == cat_lower
                    || p.categories.iter().any(|c| c.to_lowercase() == cat_lower)
            });
        }

        // 3. Author Filter
        if let Some(ref author) = query.author {
            let author_lower = author.to_lowercase();
            list.retain(|p| {
                p.authors.iter().any(|a| a.to_lowercase().contains(&author_lower))
            });
        }

        // 4. Sorting
        let sort_order = query.sort.as_deref().unwrap_or("newest");
        match sort_order {
            "oldest" => list.sort_by(|a, b| a.published.cmp(&b.published)),
            "title" => list.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase())),
            _ => list.sort_by(|a, b| b.published.cmp(&a.published)), // default newest
        }

        // 5. Pagination
        let total = list.len();
        let limit = query.limit.unwrap_or(20).clamp(1, 100);
        let page = query.page.unwrap_or(1).max(1);
        let total_pages = if total == 0 { 1 } else { (total + limit - 1) / limit };

        let offset = (page - 1) * limit;
        let items = if offset >= total {
            Vec::new()
        } else {
            list.into_iter().skip(offset).take(limit).collect()
        };

        PaginatedResponse {
            items,
            total,
            page,
            limit,
            total_pages,
        }
    }

    pub async fn get(&self, id: &str) -> Option<Paper> {
        let papers_guard = self.papers.read().await;
        papers_guard.get(id).cloned()
    }

    pub async fn create(&self, payload: CreatePaperPayload) -> Result<Paper, AppError> {
        if payload.title.trim().is_empty() {
            return Err(AppError::BadRequest("Title cannot be empty".to_string()));
        }
        if payload.authors.is_empty() {
            return Err(AppError::BadRequest("At least one author is required".to_string()));
        }
        if payload.categories.is_empty() {
            return Err(AppError::BadRequest("At least one category is required".to_string()));
        }

        let mut papers_guard = self.papers.write().await;

        let id = match payload.id {
            Some(custom_id) => {
                let trimmed = custom_id.trim().to_string();
                if trimmed.is_empty() {
                    return Err(AppError::BadRequest("Paper ID cannot be empty if specified".to_string()));
                }
                if papers_guard.contains_key(&trimmed) {
                    return Err(AppError::Conflict(format!("Paper with ID '{}' already exists", trimmed)));
                }
                trimmed
            }
            None => {
                format!("gen.{}", &Uuid::new_v4().to_string()[..8])
            }
        };

        let primary_category = payload
            .primary_category
            .unwrap_or_else(|| payload.categories.first().cloned().unwrap_or_else(|| "cs.AI".to_string()));

        let published = payload
            .published
            .unwrap_or_else(|| Utc::now().to_rfc3339());

        let arxiv_url = payload
            .arxiv_url
            .unwrap_or_else(|| format!("https://arxiv.org/abs/{}", id));

        let pdf_url = payload
            .pdf_url
            .unwrap_or_else(|| format!("https://arxiv.org/pdf/{}.pdf", id));

        let paper = Paper {
            id: id.clone(),
            title: payload.title.trim().to_string(),
            authors: payload.authors.into_iter().map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect(),
            summary: payload.summary.trim().to_string(),
            categories: payload.categories,
            primary_category,
            published,
            updated: None,
            doi: payload.doi,
            pdf_url,
            arxiv_url,
            comment: payload.comment,
            journal_ref: payload.journal_ref,
        };

        papers_guard.insert(id, paper.clone());
        Ok(paper)
    }

    pub async fn update(&self, id: &str, payload: UpdatePaperPayload) -> Result<Paper, AppError> {
        let mut papers_guard = self.papers.write().await;

        let paper = papers_guard
            .get_mut(id)
            .ok_or_else(|| AppError::NotFound(format!("Paper with ID '{}' not found", id)))?;

        if let Some(title) = payload.title {
            let trimmed = title.trim();
            if trimmed.is_empty() {
                return Err(AppError::BadRequest("Title cannot be empty".to_string()));
            }
            paper.title = trimmed.to_string();
        }

        if let Some(authors) = payload.authors {
            let filtered: Vec<String> = authors.into_iter().map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect();
            if filtered.is_empty() {
                return Err(AppError::BadRequest("At least one author is required".to_string()));
            }
            paper.authors = filtered;
        }

        if let Some(summary) = payload.summary {
            paper.summary = summary.trim().to_string();
        }

        if let Some(categories) = payload.categories {
            if categories.is_empty() {
                return Err(AppError::BadRequest("Categories cannot be empty".to_string()));
            }
            paper.categories = categories;
        }

        if let Some(primary_category) = payload.primary_category {
            paper.primary_category = primary_category;
        }

        if payload.doi.is_some() {
            paper.doi = payload.doi;
        }

        if let Some(pdf_url) = payload.pdf_url {
            paper.pdf_url = pdf_url;
        }

        if let Some(arxiv_url) = payload.arxiv_url {
            paper.arxiv_url = arxiv_url;
        }

        if payload.comment.is_some() {
            paper.comment = payload.comment;
        }

        if payload.journal_ref.is_some() {
            paper.journal_ref = payload.journal_ref;
        }

        paper.updated = Some(Utc::now().to_rfc3339());

        Ok(paper.clone())
    }

    pub async fn delete(&self, id: &str) -> Result<Paper, AppError> {
        let mut papers_guard = self.papers.write().await;
        papers_guard
            .remove(id)
            .ok_or_else(|| AppError::NotFound(format!("Paper with ID '{}' not found", id)))
    }

    pub async fn stats(&self) -> StoreStats {
        let papers_guard = self.papers.read().await;
        let mut categories_count = HashMap::new();
        let mut unique_authors = HashSet::new();

        for paper in papers_guard.values() {
            for cat in &paper.categories {
                *categories_count.entry(cat.clone()).or_insert(0) += 1;
            }
            for author in &paper.authors {
                unique_authors.insert(author.clone());
            }
        }

        StoreStats {
            total_papers: papers_guard.len(),
            categories_count,
            total_authors: unique_authors.len(),
        }
    }

    pub async fn count(&self) -> usize {
        let papers_guard = self.papers.read().await;
        papers_guard.len()
    }

    pub async fn reset(&self) {
        let mut papers_guard = self.papers.write().await;
        *papers_guard = Self::initial_seeds();
    }
}
