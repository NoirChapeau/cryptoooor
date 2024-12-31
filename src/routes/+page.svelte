<script>
  import { onMount } from 'svelte';
  import FileEncryptor from '$lib/components/FileEncryptor.svelte';
  import Features from '$lib/components/Features.svelte';
  import SocialLinks from '$lib/components/SocialLinks.svelte';
  import Creator from '$lib/components/Creator.svelte';
  import KofiButton from '$lib/components/KofiButton.svelte';
  import { fade } from 'svelte/transition';
</script>

<svelte:head>
  <title>Cryptoooor - Secure File Encryption in Your Browser</title>
  <meta name="description" content="Free, open-source file encryption tool using AES-GCM. Encrypt your files securely in your browser with military-grade encryption. No data collection, no servers, just privacy." />
  <meta property="og:title" content="Cryptoooor - Secure File Encryption" />
  <meta property="og:description" content="Free browser-based file encryption with military-grade security. Your files never leave your device." />
  <meta property="og:type" content="website" />
  <meta property="og:image" content="/og-image.jpg" />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="keywords" content="file encryption, privacy tool, AES-GCM, browser encryption, secure files, WebAssembly, Rust encryption" />
  <link rel="canonical" href="https://cryptoooor.com" />
</svelte:head>

<div class="container" in:fade>
    <header>
        <h1>Secure File Encryption</h1>
    </header>
    <section aria-label="file-encryption">
        <FileEncryptor />
    </section>
    <section aria-label="social-links">
        <SocialLinks />
    </section>
    <section aria-label="creator">
        <Creator />
        <KofiButton />
    </section>
    <section aria-label="features">
        <Features />
    </section>
</div>

<style>
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }
</style>
