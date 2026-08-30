import "./App.css";

function App() {
  return (
    <div className="app">
      <aside className="sidebar">
        <div className="brand">
          <h1>FileM4ster</h1>
          <span>v0.1</span>
        </div>

        <nav className="navigation">
          <button className="nav-item active">New Transfer</button>
          <button className="nav-item">Jobs</button>
          <button className="nav-item">Connections</button>
          <button className="nav-item">Settings</button>
        </nav>
      </aside>

      <main className="content">
        <header className="page-header">
          <div>
            <p className="eyebrow">TRANSFER</p>
            <h2>New Transfer</h2>
          </div>
        </header>

        <section className="transfer-card">
          <div className="field">
            <label>Source</label>

            <div className="path-row">
              <div className="path-placeholder">Select source...</div>
              <button>Browse</button>
            </div>
          </div>

          <div className="transfer-arrow">↓</div>

          <div className="field">
            <label>Destination</label>

            <div className="path-row">
              <div className="path-placeholder">Select destination...</div>
              <button>Browse</button>
            </div>
          </div>

          <div className="transfer-options">
            <div>
              <span className="option-label">Mode</span>
              <strong>Safe Copy</strong>
            </div>

            <div>
              <span className="option-label">Verification</span>
              <strong>Quick</strong>
            </div>
          </div>

          <div className="actions">
            <button className="secondary-button">Preview</button>
            <button className="primary-button">Start Copy</button>
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;