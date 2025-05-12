<script lang="ts">
  export let bookInfo: {
    file_name: string;
    title: string;
    author: string;
    genre: string;
    publish_year: string;
  };
  export let selectedBookDetails: { [key: string]: any } | null;
  export let jsonOutput: string;
  export let onExportJson: () => Promise<void>;
  export let onCopyJson: () => Promise<void>;
  export let onCopyDetailsJson: () => void;
  export let onBack: () => void;
</script>

<div class="container mx-auto p-4">
  <div class="flex justify-between items-center mb-6">
    <h2 class="text-2xl font-bold text-gray-800">Book Details</h2>
    <button
      on:click={onBack}
      class="bg-gray-500 text-white px-4 py-2 rounded-lg hover:bg-gray-600 transition duration-300 cursor-pointer"
    >
      Back
    </button>
  </div>

  <!-- <div class="bg-white rounded-lg shadow-md p-6 mb-6"> -->
  <!--   <h3 class="text-xl font-semibold text-gray-800 mb-4">Edit Metadata</h3> -->
  <!--   <div class="grid grid-cols-1 md:grid-cols-2 gap-4"> -->
  <!--     <div> -->
  <!--       <label class="block mb-1 text-gray-700">File Name</label> -->
  <!--       <input -->
  <!--         type="text" -->
  <!--         bind:value={bookInfo.file_name} -->
  <!--         class="border p-2 w-full rounded bg-gray-100" -->
  <!--         readonly -->
  <!--       /> -->
  <!--     </div> -->
  <!--     <div> -->
  <!--       <label class="block mb-1 text-gray-700">Title</label> -->
  <!--       <input -->
  <!--         type="text" -->
  <!--         bind:value={bookInfo.title} -->
  <!--         class="border p-2 w-full rounded" -->
  <!--       /> -->
  <!--     </div> -->
  <!--     <div> -->
  <!--       <label class="block mb-1 text-gray-700">Author</label> -->
  <!--       <input -->
  <!--         type="text" -->
  <!--         bind:value={bookInfo.author} -->
  <!--         class="border p-2 w-full rounded" -->
  <!--       /> -->
  <!--     </div> -->
  <!--     <div> -->
  <!--       <label class="block mb-1 text-gray-700">Genre</label> -->
  <!--       <input -->
  <!--         type="text" -->
  <!--         bind:value={bookInfo.genre} -->
  <!--         class="border p-2 w-full rounded" -->
  <!--       /> -->
  <!--     </div> -->
  <!--     <div> -->
  <!--       <label class="block mb-1 text-gray-700">Publish Year</label> -->
  <!--       <input -->
  <!--         type="text" -->
  <!--         bind:value={bookInfo.publish_year} -->
  <!--         class="border p-2 w-full rounded" -->
  <!--       /> -->
  <!--     </div> -->
  <!--   </div> -->
  <!--   <div class="flex gap-4 mt-6"> -->
  <!--     <button -->
  <!--       on:click={onExportJson} -->
  <!--       class="bg-green-600 text-white px-4 py-2 rounded-lg hover:bg-green-700 transition duration-300" -->
  <!--     > -->
  <!--       Export to JSON -->
  <!--     </button> -->
  <!--     {#if jsonOutput} -->
  <!--       <button -->
  <!--         on:click={onCopyJson} -->
  <!--         class="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition duration-300" -->
  <!--       > -->
  <!--         Copy Metadata JSON -->
  <!--       </button> -->
  <!--     {/if} -->
  <!--   </div> -->
  <!--   {#if jsonOutput} -->
  <!--     <pre -->
  <!--       class="bg-gray-100 p-4 mt-4 rounded max-h-60 overflow-auto">{jsonOutput}</pre> -->
  <!--   {/if} -->
  <!-- </div> -->

  {#if selectedBookDetails}
    <div class="bg-white rounded-lg shadow-md p-6">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-xl font-semibold text-gray-800">
          Detailed Book Information
        </h3>
        <button
          on:click={onCopyDetailsJson}
          class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition duration-300 cursor-pointer"
        >
          Copy Detailed JSON
        </button>
      </div>
      <pre
        class="bg-gray-100 p-4 rounded max-h-120 overflow-auto">{JSON.stringify(
          selectedBookDetails,
          null,
          2,
        )}</pre>
    </div>
  {/if}
</div>
