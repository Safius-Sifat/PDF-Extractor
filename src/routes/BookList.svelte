<script lang="ts">
  export let searchResults: Array<{
    id: string;
    title: string;
    authors: string;
    publish_year: string;
    thumbnail: string;
    selfLink: string;
  }>;
  export let onSelectBook: (book: any) => Promise<void>;
  export let onBack: () => void;
</script>

<div class="container mx-auto p-4">
  <div class="flex justify-between items-center mb-6">
    <h2 class="text-2xl font-bold text-gray-800">Search Results</h2>
    <button
      on:click={onBack}
      class="bg-gray-500 text-white px-4 py-2 rounded-lg hover:bg-gray-600 transition duration-300 cursor-pointer"
    >
      Back
    </button>
  </div>
  {#if searchResults.length > 0}
    <div
      class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6"
    >
      {#each searchResults as book}
        <div
          class="bg-white rounded-lg shadow-md overflow-hidden hover:shadow-xl transition duration-300 cursor-pointer"
          on:click={() => onSelectBook(book)}
        >
          <img
            src={book.thumbnail}
            alt={book.title}
            class="w-full h-48 object-cover"
          />
          <div class="p-4">
            <h3 class="text-lg font-semibold text-gray-800 line-clamp-2">
              {book.title}
            </h3>
            <p class="text-gray-600 text-sm">
              {book.authors || "Unknown Author"}
            </p>
            <p class="text-gray-500 text-sm">
              {book.publish_year || "Unknown Year"}
            </p>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <p class="text-gray-600 text-center">No books found.</p>
  {/if}
</div>
