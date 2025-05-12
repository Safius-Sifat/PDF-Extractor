<script lang="ts">
  export let onPickPdf: () => Promise<void>;
</script>

<div class="flex flex-col items-center justify-center min-h-screen text-center">
  <h1
    class="text-5xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-purple-600 mb-8"
  >
    PDF Extractor
  </h1>
  <button
    on:click={onPickPdf}
    class="bg-blue-600 text-white px-6 py-3 rounded-lg shadow-lg hover:bg-blue-700 transition duration-300 text-lg cursor-pointer"
  >
    Pick PDF
  </button>
</div>
