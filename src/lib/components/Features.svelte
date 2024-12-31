<section class="features">
  <div class="intro-text">
    <h2>Security Shouldn't Be Complicated</h2>
    <p>In a world where privacy is increasingly scarce, Cryptoooor stands as a beacon of simplicity and security. Built with Rust and WebAssembly, it brings professional-grade encryption right to your browser — no installations, no accounts, no complexity.</p>
    <p>Your files never leave your device. Using the same AES-GCM encryption trusted by banks and governments, Cryptoooor ensures your data remains yours alone. Simple, fast, and completely private.</p>
    <div class="highlight-bar">
      <span>Client-Side Security</span>
      <span>•</span>
      <span>AES-GCM Encryption</span>
      <span>•</span>
      <span>Zero Data Collection</span>
    </div>
  </div>

  <div class="why-section">
    <h2>Why Cryptoooor?</h2>
    <div class="benefits">
      {#each benefits as benefit}
        <div class="benefit">
          <h4>{benefit.title}</h4>
          <p>{benefit.description}</p>
        </div>
      {/each}
    </div>
  </div>

    <div class="tech-items">
     {#each techItems as item}
       <a href={item.url} class="tech-item" target="_blank" rel="noopener noreferrer">
         {item.name}
       </a>
     {/each}
    </div>
</section>

<script>
  const benefits = [
    {
      title: "Privacy First",
      description: "No tracking, no analytics, no data collection. Just pure, client-side encryption."
    },
    {
      title: "Built for Speed",
      description: "Encrypt files of any size with near-native performance, thanks to WebAssembly optimization."
    },
    {
      title: "Easy to Use",
      description: "No complicated settings or confusing options. Select a file, encrypt or decrypt, done."
    }
  ];

const techItems = [
 { 
   name: "Rust", 
   url: "https://www.rust-lang.org/"
 },
 { 
   name: "WebAssembly",
   url: "https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm"
 },
 { 
   name: "AES-GCM", 
   url: "https://www.rfc-editor.org/rfc/rfc5288" 
 }
];
</script>

<style>
    .tech-items {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    justify-content: center;
    }

    .tech-item {
    background: linear-gradient(45deg, var(--primary), var(--accent));
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.9rem;
    color: var(--text);
    text-decoration: none;
    transition: all 0.3s ease;
    }

    .tech-item:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(251, 73, 52, 0.2);
    }
</style>
