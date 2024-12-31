<script>
  import { onMount } from 'svelte';
  import init, { encrypt_file, decrypt_file, generate_key } from "../pkg/cryptoooor_wasm.js";

  let fileBuffer = null;
  let originalFileName = null;
  let status = "Loading...";
  let isFileSelected = false;
  let generatedKey = null;

  onMount(async () => {
    await init();
    status = "Select a file to begin";
  });

  function uint8ArrayToHex(arr) {
    return Array.from(arr).map(b => b.toString(16).padStart(2, '0')).join('');
  }

  function hexToUint8Array(hexStr) {
    const result = new Uint8Array(hexStr.length / 2);
    for (let i = 0; i < hexStr.length; i += 2) {
      result[i / 2] = parseInt(hexStr.substr(i, 2), 16);
    }
    return result;
  }

  async function handleFileSelect(event) {
    const file = event.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        fileBuffer = new Uint8Array(e.target.result);
        originalFileName = file.name.split('.').slice(0, -1).join('.');
        isFileSelected = true;
        status = `Selected file: ${file.name}`;
      };
      reader.readAsArrayBuffer(file);
    }
  }

  async function handleGenerateKey() {
    const key = generate_key();
    const keyHex = uint8ArrayToHex(key);
    status = `Here's your key: ${keyHex}`;
    generatedKey = key;
  }

  function downloadFile(data, filename) {
    const blob = new Blob([data], { type: 'application/octet-stream' });
    const url = window.URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    link.click();
    window.URL.revokeObjectURL(url);
  }

  async function handleEncrypt() {
    if (!fileBuffer) {
      alert("Please select a file first.");
      return;
    }

    const keyHex = generatedKey ? uint8ArrayToHex(generatedKey) : prompt("Enter encryption key (hex):");

    try {
      const keyBytes = hexToUint8Array(keyHex);
      if (keyBytes.length !== 32) {
        throw new Error("Key must be 32 bytes long.");
      }

      const encryptedData = encrypt_file(fileBuffer, keyBytes);
      downloadFile(encryptedData, `${originalFileName}.bin`);
      status = "File encrypted!";
    } catch (error) {
      console.error("Encryption error: ", error);
      status = "Encryption failed: " + error.message;
    }
  }

  async function handleDecrypt() {
    if (!fileBuffer || !originalFileName) {
      alert("Please select a file first.");
      return;
    }

    const keyHex = generatedKey ? uint8ArrayToHex(generatedKey) : prompt("Enter your decryption key (hex):");

    try {
      const keyBytes = hexToUint8Array(keyHex);
      if (keyBytes.length !== 32) {
        throw new Error("Key must be 32 bytes long.");
      }

      const decryptedData = await decrypt_file(fileBuffer, keyBytes);
      const extension = prompt("Enter the desired file extension (e.g., pdf, txt, docx):", "");
      const decryptedFileName = extension ? `${originalFileName}.${extension}` : originalFileName;
      downloadFile(decryptedData, decryptedFileName);
      status = "File decrypted!";
    } catch (error) {
      console.error("Decryption error: ", error);
      status = "Decryption failed: " + error.message;
    }
  }
</script>

<div class="file-encryptor">
  <svg class="logo" viewBox="0 0 24 24" width="80" height="80">
    <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z" 
          fill="currentColor"/>
  </svg>

  <h1>Cryptoooor</h1>
  <p class="subtitle">Secure file encryption and decryption in your browser</p>

  <div class="file-input-wrapper">
    <label for="fileInput" class="custom-file-input">
      <svg viewBox="0 0 24 24" width="16" height="16">
        <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" fill="currentColor"/>
      </svg>
      Choose File
    </label>
    <input 
      type="file" 
      id="fileInput" 
      class="hidden-file-input"
      on:change={handleFileSelect}
    >
    <p class="file-status">{status}</p>
  </div>

  <div class="button-group">
    <button 
      class="btn" 
      disabled={!isFileSelected}
      on:click={handleEncrypt}
    >
      <svg viewBox="0 0 24 24" width="16" height="16">
        <path d="M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zM9 6c0-1.66 1.34-3 3-3s3 1.34 3 3v2H9V6z" fill="currentColor"/>
      </svg>
      Encrypt File
    </button>
    <button 
      class="btn" 
      disabled={!isFileSelected}
      on:click={handleDecrypt}
    >
      <svg viewBox="0 0 24 24" width="16" height="16">
        <path d="M12 17c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm6-9h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6h1.9c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2z" fill="currentColor"/>
      </svg>
      Decrypt File
    </button>
    <button 
      class="btn"
      on:click={handleGenerateKey}
    >
      <svg viewBox="0 0 24 24" width="16" height="16">
        <path d="M22 19h-6v-4h-2.68c-1.14 2.42-3.6 4-6.32 4-3.86 0-7-3.14-7-7s3.14-7 7-7c2.72 0 5.17 1.58 6.32 4H24v6h-2v4zm-4-2h2v-4h2v-2H11.94l-.23-.67C11.01 8.34 9.11 7 7 7c-2.76 0-5 2.24-5 5s2.24 5 5 5c2.11 0 4.01-1.34 4.71-3.33l.23-.67H18v4zM7 15c-1.65 0-3-1.35-3-3s1.35-3 3-3 3 1.35 3 3-1.35 3-3 3z" fill="currentColor"/>
      </svg>
      Generate Key
    </button>
  </div>
</div>
