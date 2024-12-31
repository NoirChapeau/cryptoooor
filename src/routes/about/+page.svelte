<script>
  import { fade } from 'svelte/transition';
  const faqs = [
    {
      question: "Is Cryptoooor really free?",
      answer: "Yes! Cryptoooor is completely free and open-source. We believe privacy is a fundamental right."
    },
    {
      question: "How secure is the encryption?",
      answer: "We use AES-GCM with 256-bit keys, the same encryption standard used by governments and financial institutions."
    },
    {
      question: "Can I encrypt any type of file?",
      answer: "Yes, Cryptoooor can encrypt any file type, from documents to images to videos."
    },
    {
      question: "Where are my files stored?",
      answer: "Nowhere! Everything happens in your browser. Files are never uploaded to any server."
    }
  ];
</script>

<svelte:head>
  <title>About Cryptoooor - Our Mission and Technology</title>
  <meta name="description" content="Learn about Cryptoooor's mission to make privacy accessible. Built with Rust and WebAssembly, using AES-GCM encryption for maximum security and performance." />
  <meta property="og:title" content="About Cryptoooor - Privacy-First Encryption" />
  <meta property="og:description" content="Discover how Cryptoooor makes military-grade encryption accessible to everyone. Built with cutting-edge technology for security and speed." />
  <meta property="og:type" content="website" />
  <meta property="og:image" content="/about-og-image.jpg" />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="keywords" content="encryption technology, privacy tools, AES-GCM encryption, WebAssembly security, Rust crypto" />
  <link rel="canonical" href="https://cryptoooor.com/about" />
</svelte:head>

<div class="about-page" in:fade>
  <section class="about-hero">
    <h1>About Cryptoooor</h1>
    <p class="subtitle">A Privacy-First File Encryption Tool</p>
    <hr>
  </section>

  <section class="about-content">
    <div class="mission">
      <h2>Our Mission</h2>
      <p>In an age where digital privacy is increasingly compromised, Cryptoooor stands as a testament to the belief that security should be accessible to everyone. Our mission is to provide military-grade encryption without the complexity that often accompanies security tools.</p>
    </div>

    <div class="self-hosted">
      <h2>Self-Hosted Version</h2>
      <p>For users who prefer complete control, we maintain a legacy Docker image of Cryptoooor:</p>
      
      <div class="code-block">
        <code>docker pull pindjouf/cryptoooor</code>
        <button class="copy-btn" on:click={() => navigator.clipboard.writeText('docker pull pindjouf/cryptoooor')}>Copy</button>
      </div>

      <div class="legacy-note">
        <h3>Note about the Docker version:</h3>
        <ul>
          <li>Core encryption functionality is identical</li>
          <li>Simplified UI with basic styling</li>
          <li>Fixed output filenames after decryption</li>
          <li>Legacy version - not actively maintained</li>
        </ul>
        
        <a href="https://registry.hub.docker.com/r/pindjouf/cryptoooor" class="docker-link" target="_blank" rel="noopener noreferrer">
          View on Docker Hub
          <svg viewBox="0 0 24 24" width="16" height="16">
            <path d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z" fill="currentColor"/>
          </svg>
        </a>
      </div>
    </div>

    <div class="faq">
      <h2>Frequently Asked Questions</h2>
      <div class="faq-grid">
        {#each faqs as {question, answer}}
          <div class="faq-item">
            <h3>{question}</h3>
            <p>{answer}</p>
          </div>
        {/each}
      </div>
    </div>
  </section>
</div>

<style>
  .about-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  .about-hero {
    text-align: center;
    margin-bottom: 4rem;
  }

  .about-hero h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .subtitle {
    font-size: 1.2rem;
    opacity: 0.8;
  }

  .about-content {
    display: flex;
    flex-direction: column;
    gap: 4rem;
  }

  .mission {
    max-width: 800px;
    margin: 0 auto;
    text-align: center;
  }

  .tech-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
    margin-top: 2rem;
  }

  .tech-card {
    background: var(--glass);
    backdrop-filter: blur(12px);
    border: 1px solid var(--glass-border);
    border-radius: 12px;
    padding: 2rem;
  }

  .tech-card h3 {
    color: var(--accent);
    margin-bottom: 1rem;
  }

  .faq-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
    margin-top: 2rem;
  }

  .faq-item {
    background: var(--glass);
    backdrop-filter: blur(12px);
    border: 1px solid var(--glass-border);
    border-radius: 12px;
    padding: 2rem;
  }

  .faq-item h3 {
    color: var(--accent);
    margin-bottom: 1rem;
  }

  .self-hosted {
    background: var(--glass);
    backdrop-filter: blur(12px);
    border: 1px solid var(--glass-border);
    border-radius: 12px;
    padding: 2rem;
  }

  .code-block {
    background: rgba(0, 0, 0, 0.2);
    padding: 1rem;
    border-radius: 8px;
    margin: 1rem 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .copy-btn {
    background: var(--accent);
    color: var(--text);
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .copy-btn:hover {
    opacity: 0.9;
  }

  .legacy-note {
    margin-top: 1.5rem;
  }

  .legacy-note ul {
    margin: 1rem 0;
  }

  .docker-link {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--accent);
    text-decoration: none;
  }

  .docker-link:hover {
    text-decoration: underline;
  }

  @media (max-width: 768px) {
    .about-hero h1 {
      font-size: 2rem;
    }

    .tech-grid, .faq-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
