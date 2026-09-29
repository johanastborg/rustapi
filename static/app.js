/**
 * arXiv Paper Storage - Axum REST API Client
 */

// Application State
let currentTab = 'papers';
let currentSearch = '';
let currentCategory = '';
let currentSort = 'newest';
let currentPage = 1;
let currentLimit = 20;

// API Playground State
let selectedEndpointKey = 'get_papers';

const ENDPOINTS = {
  get_papers: {
    method: 'GET',
    url: '/api/papers',
    hasBody: false,
    body: '',
    curl: 'curl -X GET http://localhost:3000/api/papers',
  },
  get_paper_id: {
    method: 'GET',
    url: '/api/papers/1706.03762',
    hasBody: false,
    body: '',
    curl: 'curl -X GET http://localhost:3000/api/papers/1706.03762',
  },
  post_paper: {
    method: 'POST',
    url: '/api/papers',
    hasBody: true,
    body: JSON.stringify(
      {
        id: "2404.09999",
        title: "Scaling Laws for Autoregressive Generative Diffusion Models",
        authors: ["Sheng Shen", "Kurt Keutzer"],
        summary: "We present empirical scaling laws for diffusion models trained with transformer backbones across computational compute budgets.",
        categories: ["cs.LG", "cs.AI", "cs.CV"],
        primary_category: "cs.LG",
        comment: "Preprint, 18 pages"
      },
      null,
      2
    ),
    curl: `curl -X POST http://localhost:3000/api/papers \\
  -H "Content-Type: application/json" \\
  -d '{"id":"2404.09999","title":"Scaling Laws...","authors":["Sheng Shen"],"summary":"...","categories":["cs.LG"]}'`,
  },
  put_paper: {
    method: 'PUT',
    url: '/api/papers/1706.03762',
    hasBody: true,
    body: JSON.stringify(
      {
        title: "Attention Is All You Need (Revised NeurIPS Classic)",
        comment: "Updated citation metadata"
      },
      null,
      2
    ),
    curl: `curl -X PUT http://localhost:3000/api/papers/1706.03762 \\
  -H "Content-Type: application/json" \\
  -d '{"title":"Attention Is All You Need (Revised NeurIPS Classic)"}'`,
  },
  delete_paper: {
    method: 'DELETE',
    url: '/api/papers/2404.09999',
    hasBody: false,
    body: '',
    curl: 'curl -X DELETE http://localhost:3000/api/papers/2404.09999',
  },
  get_stats: {
    method: 'GET',
    url: '/api/stats',
    hasBody: false,
    body: '',
    curl: 'curl -X GET http://localhost:3000/api/stats',
  },
  get_health: {
    method: 'GET',
    url: '/health',
    hasBody: false,
    body: '',
    curl: 'curl -X GET http://localhost:3000/health',
  },
  post_reset: {
    method: 'POST',
    url: '/api/papers/reset',
    hasBody: false,
    body: '',
    curl: 'curl -X POST http://localhost:3000/api/papers/reset',
  },
};

// Initialization
document.addEventListener('DOMContentLoaded', () => {
  initHealthCheck();
  loadStats();
  loadPapers();
  setupSearchInput();
  selectEndpoint('get_papers');
});

// Tab Switcher
function switchTab(tab) {
  currentTab = tab;
  document.getElementById('nav-tab-papers').classList.toggle('active', tab === 'papers');
  document.getElementById('nav-tab-api').classList.toggle('active', tab === 'api');

  document.getElementById('view-papers').style.display = tab === 'papers' ? 'block' : 'none';
  document.getElementById('view-api').style.display = tab === 'api' ? 'block' : 'none';
}

// Health Check
async function initHealthCheck() {
  const check = async () => {
    try {
      const res = await fetch('/health');
      if (res.ok) {
        const data = await res.json();
        const statusEl = document.getElementById('server-status-text');
        statusEl.textContent = `Axum Online (${data.paper_count} papers)`;
        document.getElementById('server-status-pill').style.borderColor = 'rgba(16, 185, 129, 0.4)';
      }
    } catch (err) {
      const statusEl = document.getElementById('server-status-text');
      statusEl.textContent = 'Server Offline';
      document.getElementById('server-status-pill').style.borderColor = 'rgba(239, 68, 68, 0.4)';
    }
  };

  check();
  setInterval(check, 10000);
}

// Load Store Stats
async function loadStats() {
  try {
    const res = await fetch('/api/stats');
    if (!res.ok) return;
    const stats = await res.json();
    document.getElementById('stat-total-papers').textContent = stats.total_papers;
    document.getElementById('stat-total-authors').textContent = stats.total_authors;
    document.getElementById('stat-total-categories').textContent = Object.keys(stats.categories_count).length;
  } catch (e) {
    console.error('Error fetching stats:', e);
  }
}

// Search & Filtering
function setupSearchInput() {
  const input = document.getElementById('paper-search-input');
  const clearBtn = document.getElementById('clear-search-btn');

  let debounceTimer;
  input.addEventListener('input', (e) => {
    currentSearch = e.target.value.trim();
    clearBtn.style.display = currentSearch ? 'block' : 'none';
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      currentPage = 1;
      loadPapers();
    }, 250);
  });
}

function clearSearch() {
  const input = document.getElementById('paper-search-input');
  input.value = '';
  currentSearch = '';
  document.getElementById('clear-search-btn').style.display = 'none';
  currentPage = 1;
  loadPapers();
}

function handleSortChange(sort) {
  currentSort = sort;
  currentPage = 1;
  loadPapers();
}

function filterCategory(cat) {
  currentCategory = cat;
  document.querySelectorAll('.cat-pill').forEach((pill) => {
    pill.classList.toggle('active', pill.dataset.category === cat);
  });
  currentPage = 1;
  loadPapers();
}

// Fetch & Render Papers
async function loadPapers() {
  const grid = document.getElementById('papers-grid');
  const countLabel = document.getElementById('results-count-text');

  const params = new URLSearchParams();
  if (currentSearch) params.append('q', currentSearch);
  if (currentCategory) params.append('category', currentCategory);
  if (currentSort) params.append('sort', currentSort);
  params.append('page', currentPage);
  params.append('limit', currentLimit);

  try {
    const res = await fetch(`/api/papers?${params.toString()}`);
    if (!res.ok) throw new Error('Failed to load papers');
    const data = await res.json();

    countLabel.textContent = `Showing ${data.items.length} of ${data.total} papers in store`;
    renderPagination(data);

    if (data.items.length === 0) {
      grid.innerHTML = `
        <div style="grid-column: 1/-1; text-align: center; padding: 4rem 1rem; color: var(--text-dim);">
          <div style="font-size: 2.5rem; margin-bottom: 0.5rem;">🔍</div>
          <h3 style="color: var(--text-main); margin-bottom: 0.5rem;">No arXiv papers match your criteria</h3>
          <p>Try searching for a different topic, author, or clear the category filter.</p>
          <button class="btn btn-ghost" style="margin-top: 1rem;" onclick="clearSearch(); filterCategory('');">Reset Filters</button>
        </div>
      `;
      return;
    }

    grid.innerHTML = data.items.map((paper) => createPaperCard(paper)).join('');
  } catch (err) {
    grid.innerHTML = `<div style="grid-column: 1/-1; color: var(--status-error); padding: 2rem;">Error connecting to API: ${err.message}</div>`;
  }
}

function createPaperCard(paper) {
  const formattedDate = paper.published ? paper.published.split('T')[0] : 'Unknown';
  const categoryPills = paper.categories
    .map((c) => `<span class="cat-tag ${c === paper.primary_category ? 'primary' : ''}">${escapeHtml(c)}</span>`)
    .join('');

  return `
    <article class="paper-card" id="paper-card-${escapeHtml(paper.id)}">
      <div>
        <div class="paper-top">
          <a href="${escapeHtml(paper.arxiv_url)}" target="_blank" rel="noopener noreferrer" class="arxiv-id-badge" title="Open on arXiv.org">
            arXiv:${escapeHtml(paper.id)} ↗
          </a>
          <div class="category-tags">
            ${categoryPills}
          </div>
        </div>

        <h3 class="paper-title">${escapeHtml(paper.title)}</h3>
        <div class="paper-authors">👤 ${escapeHtml(paper.authors.join(', '))}</div>
        
        <p class="paper-summary" id="summary-${escapeHtml(paper.id)}">${escapeHtml(paper.summary)}</p>
        <button class="btn-toggle-summary" onclick="toggleSummary('${escapeHtml(paper.id)}')">Read abstract</button>
      </div>

      <div class="paper-footer">
        <div class="paper-meta">
          <span class="paper-date">Published: ${formattedDate}</span>
          ${paper.comment ? `<span style="max-width: 200px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;" title="${escapeHtml(paper.comment)}">📌 ${escapeHtml(paper.comment)}</span>` : ''}
        </div>

        <div class="paper-actions">
          <a href="${escapeHtml(paper.pdf_url)}" target="_blank" rel="noopener noreferrer" class="btn-icon" title="View PDF">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="16" y1="13" x2="8" y2="13"></line><line x1="16" y1="17" x2="8" y2="17"></line><polyline points="10 9 9 9 8 9"></polyline></svg>
          </a>
          <button class="btn-icon" title="Edit paper metadata" onclick="openEditModal('${escapeHtml(paper.id)}')">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>
          </button>
          <button class="btn-icon danger" title="Delete paper from storage" onclick="deletePaper('${escapeHtml(paper.id)}')">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
          </button>
        </div>
      </div>
    </article>
  `;
}

function toggleSummary(id) {
  const card = document.getElementById(`paper-card-${id}`);
  if (card) {
    card.classList.toggle('expanded');
    const btn = card.querySelector('.btn-toggle-summary');
    btn.textContent = card.classList.contains('expanded') ? 'Collapse abstract' : 'Read abstract';
  }
}

function renderPagination(data) {
  const container = document.getElementById('pagination-controls');
  if (data.total_pages <= 1) {
    container.innerHTML = '';
    return;
  }

  let html = '';
  if (data.page > 1) {
    html += `<button class="btn btn-ghost" onclick="goToPage(${data.page - 1})">Prev</button>`;
  }
  html += `<span style="font-size: 0.8rem; margin: 0 0.5rem;">Page ${data.page} of ${data.total_pages}</span>`;
  if (data.page < data.total_pages) {
    html += `<button class="btn btn-ghost" onclick="goToPage(${data.page + 1})">Next</button>`;
  }
  container.innerHTML = html;
}

function goToPage(page) {
  currentPage = page;
  loadPapers();
  window.scrollTo({ top: 350, behavior: 'smooth' });
}

// Create Paper Modal & Handler
function openCreateModal() {
  document.getElementById('create-modal').style.display = 'flex';
}

function closeCreateModal() {
  document.getElementById('create-modal').style.display = 'none';
  document.getElementById('create-paper-form').reset();
}

async function handleCreatePaper(e) {
  e.preventDefault();
  const id = document.getElementById('create-id').value.trim();
  const title = document.getElementById('create-title').value.trim();
  const authorsRaw = document.getElementById('create-authors').value.trim();
  const primaryCategory = document.getElementById('create-primary-cat').value;
  const categoriesRaw = document.getElementById('create-categories').value.trim();
  const summary = document.getElementById('create-summary').value.trim();
  const comment = document.getElementById('create-comment').value.trim();
  const doi = document.getElementById('create-doi').value.trim();

  const authors = authorsRaw.split(',').map((a) => a.trim()).filter(Boolean);
  const additionalCategories = categoriesRaw.split(',').map((c) => c.trim()).filter(Boolean);
  const categories = Array.from(new Set([primaryCategory, ...additionalCategories]));

  const payload = {
    title,
    authors,
    summary,
    categories,
    primary_category: primaryCategory,
  };

  if (id) payload.id = id;
  if (comment) payload.comment = comment;
  if (doi) payload.doi = doi;

  try {
    const res = await fetch('/api/papers', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });

    if (!res.ok) {
      const errData = await res.json();
      throw new Error(errData.error || 'Failed to save paper');
    }

    const created = await res.json();
    showToast(`Paper '${created.title}' added successfully!`, 'success');
    closeCreateModal();
    loadPapers();
    loadStats();
    initHealthCheck();
  } catch (err) {
    showToast(err.message, 'error');
  }
}

// Edit Paper Modal & Handler
async function openEditModal(id) {
  try {
    const res = await fetch(`/api/papers/${id}`);
    if (!res.ok) throw new Error('Paper not found');
    const paper = await res.json();

    document.getElementById('edit-id').value = paper.id;
    document.getElementById('edit-paper-id-display').textContent = paper.id;
    document.getElementById('edit-title').value = paper.title;
    document.getElementById('edit-authors').value = paper.authors.join(', ');
    document.getElementById('edit-primary-cat').value = paper.primary_category;
    document.getElementById('edit-categories').value = paper.categories.join(', ');
    document.getElementById('edit-summary').value = paper.summary;
    document.getElementById('edit-comment').value = paper.comment || '';

    document.getElementById('edit-modal').style.display = 'flex';
  } catch (err) {
    showToast(err.message, 'error');
  }
}

function closeEditModal() {
  document.getElementById('edit-modal').style.display = 'none';
}

async function handleUpdatePaper(e) {
  e.preventDefault();
  const id = document.getElementById('edit-id').value;
  const title = document.getElementById('edit-title').value.trim();
  const authorsRaw = document.getElementById('edit-authors').value.trim();
  const primary_category = document.getElementById('edit-primary-cat').value;
  const categoriesRaw = document.getElementById('edit-categories').value.trim();
  const summary = document.getElementById('edit-summary').value.trim();
  const comment = document.getElementById('edit-comment').value.trim();

  const authors = authorsRaw.split(',').map((a) => a.trim()).filter(Boolean);
  const categories = categoriesRaw.split(',').map((c) => c.trim()).filter(Boolean);

  const payload = {
    title,
    authors,
    summary,
    categories,
    primary_category,
    comment: comment || null,
  };

  try {
    const res = await fetch(`/api/papers/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });

    if (!res.ok) {
      const err = await res.json();
      throw new Error(err.error || 'Failed to update paper');
    }

    showToast(`Paper '${id}' updated!`, 'success');
    closeEditModal();
    loadPapers();
  } catch (err) {
    showToast(err.message, 'error');
  }
}

// Delete Paper
async function deletePaper(id) {
  if (!confirm(`Are you sure you want to delete paper '${id}' from mock storage?`)) {
    return;
  }

  try {
    const res = await fetch(`/api/papers/${id}`, { method: 'DELETE' });
    if (!res.ok) {
      const err = await res.json();
      throw new Error(err.error || 'Failed to delete paper');
    }
    showToast(`Paper '${id}' deleted`, 'success');
    loadPapers();
    loadStats();
    initHealthCheck();
  } catch (err) {
    showToast(err.message, 'error');
  }
}

// Reset Storage
async function confirmResetStore() {
  if (!confirm('Reset in-memory storage to the default initial arXiv seed papers?')) {
    return;
  }

  try {
    const res = await fetch('/api/papers/reset', { method: 'POST' });
    if (!res.ok) throw new Error('Reset failed');
    const data = await res.json();
    showToast(data.message, 'success');
    loadPapers();
    loadStats();
    initHealthCheck();
  } catch (err) {
    showToast(err.message, 'error');
  }
}

// API Playground Logic
function selectEndpoint(key) {
  selectedEndpointKey = key;
  const config = ENDPOINTS[key];

  document.querySelectorAll('.api-endpoint-item').forEach((item) => {
    item.classList.remove('active');
  });
  event?.currentTarget?.classList.add('active');

  const methodBadge = document.getElementById('console-method');
  methodBadge.textContent = config.method;
  methodBadge.className = `http-badge ${config.method.toLowerCase()}`;

  document.getElementById('console-url-input').value = config.url;

  const bodyWrapper = document.getElementById('console-body-wrapper');
  if (config.hasBody) {
    bodyWrapper.style.display = 'block';
    document.getElementById('console-json-payload').value = config.body;
  } else {
    bodyWrapper.style.display = 'none';
  }

  document.getElementById('curl-code-snippet').textContent = config.curl;
}

async function executeApiRequest() {
  const method = document.getElementById('console-method').textContent;
  const url = document.getElementById('console-url-input').value.trim();
  const outputCode = document.getElementById('response-json-output');
  const statusBadge = document.getElementById('response-status-badge');
  const timeBadge = document.getElementById('response-time-badge');

  outputCode.textContent = '// Sending request...';

  const startTime = performance.now();

  try {
    const options = { method };
    if (method === 'POST' || method === 'PUT') {
      const rawPayload = document.getElementById('console-json-payload').value;
      options.headers = { 'Content-Type': 'application/json' };
      options.body = rawPayload;
    }

    const res = await fetch(url, options);
    const duration = Math.round(performance.now() - startTime);
    timeBadge.textContent = `${duration} ms`;

    statusBadge.textContent = `HTTP ${res.status} ${res.statusText}`;
    statusBadge.className = `badge-status ${res.ok ? 'success' : 'error'}`;

    const text = await res.text();
    let jsonFormatted;
    try {
      jsonFormatted = JSON.stringify(JSON.parse(text), null, 2);
    } catch {
      jsonFormatted = text;
    }

    outputCode.textContent = jsonFormatted;

    // Refresh papers list in background if mutation occurred
    if (['POST', 'PUT', 'DELETE'].includes(method)) {
      loadPapers();
      loadStats();
      initHealthCheck();
    }
  } catch (err) {
    const duration = Math.round(performance.now() - startTime);
    timeBadge.textContent = `${duration} ms`;
    statusBadge.textContent = 'Error';
    statusBadge.className = 'badge-status error';
    outputCode.textContent = `Network / Execution Error:\n${err.message}`;
  }
}

function copyCurl() {
  const curl = document.getElementById('curl-code-snippet').textContent;
  navigator.clipboard.writeText(curl).then(() => {
    showToast('cURL command copied to clipboard!', 'success');
  });
}

// Utility: Toast notifications
function showToast(message, type = 'info') {
  const container = document.getElementById('toast-container');
  const toast = document.createElement('div');
  toast.className = `toast ${type}`;
  toast.innerHTML = `
    <span>${type === 'success' ? '✓' : '⚠️'}</span>
    <span>${escapeHtml(message)}</span>
  `;
  container.appendChild(toast);

  setTimeout(() => {
    toast.style.opacity = '0';
    toast.style.transform = 'translateY(10px)';
    toast.style.transition = 'all 0.25s ease';
    setTimeout(() => toast.remove(), 250);
  }, 3500);
}

function escapeHtml(str) {
  if (typeof str !== 'string') return str;
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}
