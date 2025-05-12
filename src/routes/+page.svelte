<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onDestroy } from "svelte";
  import Home from "./Home.svelte";
  import BookList from "./BookList.svelte";
  import BookDetails from "./BookDetails.svelte";

  type Page = "home" | "bookList" | "bookDetails";

  interface BookInfo {
    file_name: string;
    title: string;
    author: string;
    genre: string;
    publish_year: string;
  }

  interface BookSearchResult {
    id: string;
    title: string;
    authors: string;
    publish_year: string;
    thumbnail: string;
    selfLink: string;
  }

  interface BookDetails {
    [key: string]: any; // Flexible to handle nested JSON
  }

  let bookInfo: BookInfo = {
    file_name: "",
    title: "",
    author: "",
    genre: "",
    publish_year: "",
  };
  let currentPage: Page = "home";
  let searchResults: BookSearchResult[] = [];
  let selectedBookDetails: BookDetails | null = null;
  let jsonOutput: string = "";
  let errorMessage: string = "";
  let isLoading: boolean = false;

  // Auto-dismiss error message after 3 seconds
  let errorTimeout: ReturnType<typeof setTimeout> | null = null;
  $: if (errorMessage) {
    if (errorTimeout) clearTimeout(errorTimeout);
    errorTimeout = setTimeout(() => {
      errorMessage = "";
    }, 3000);
  }

  onDestroy(() => {
    if (errorTimeout) clearTimeout(errorTimeout);
  });

  async function searchBookInfo(title: string): Promise<BookSearchResult[]> {
    try {
      const response = await fetch(
        `https://www.googleapis.com/books/v1/volumes?q=${encodeURIComponent(title)}`,
      );
      if (!response.ok) throw new Error("Failed to fetch book info");
      const data = await response.json();

      return (data.items || []).map((item: any) => ({
        id: item.id,
        title: item.volumeInfo.title || title,
        authors: item.volumeInfo.authors
          ? item.volumeInfo.authors.join(", ")
          : "",
        publish_year: item.volumeInfo.publishedDate || "",
        thumbnail:
          item.volumeInfo.imageLinks?.thumbnail ||
          "https://via.placeholder.com/128x192?text=No+Cover",
        selfLink: item.selfLink || "",
      }));
    } catch (error) {
      console.error("API Error:", error);
      return [];
    }
  }
  async function fetchBookDetails(selfLink: string): Promise<BookDetails> {
    try {
      const response = await fetch(selfLink);
      if (!response.ok) throw new Error("Failed to fetch book details");
      return await response.json();
    } catch (error) {
      console.error("Details API Error:", error);
      return {};
    }
  }
  async function handleBookSelect(book: BookSearchResult): Promise<void> {
    isLoading = true;
    try {
      errorMessage = "";
      selectedBookDetails = await fetchBookDetails(book.selfLink);
      bookInfo = {
        file_name: bookInfo.file_name,
        title: book.title,
        author: book.authors,
        genre: selectedBookDetails?.volumeInfo?.categories?.[0] || "",
        publish_year: book.publish_year,
      };
      currentPage = "bookDetails";
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isLoading = false;
    }
  }
  async function pickAndProcessPdf() {
    try {
      isLoading = true;
      errorMessage = "";
      const file = await open({
        multiple: false,
        directory: false,
      });
      console.log(file);
      const metadata: BookInfo = await invoke("extract_pdf_metadata", {
        filePath: file,
      });
      searchResults = await searchBookInfo(metadata.title);
      bookInfo = {
        file_name: metadata.file_name,
        title: metadata.title,
        author: metadata.author || "",
        genre: "",
        publish_year: "",
      };
      if (searchResults.length > 0) {
        currentPage = "bookList";
      }
    } catch (error) {
      console.error(error);
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isLoading = false;
    }
  }

  async function exportJson(): Promise<void> {
    try {
      errorMessage = "";
      jsonOutput = await invoke("export_to_json", { bookInfo });
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  async function copyJson(): Promise<void> {
    try {
      errorMessage = "";
      await writeText(jsonOutput);
      alert("JSON copied to clipboard!");
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    }
  }
  function copyDetailsJson(): void {
    if (selectedBookDetails) {
      writeText(JSON.stringify(selectedBookDetails, null, 2))
        .then(() => alert("Detailed JSON copied to clipboard!"))
        .catch((error) => {
          errorMessage = error instanceof Error ? error.message : String(error);
        });
    }
  }

  function goBack(): void {
    if (currentPage === "bookDetails") {
      currentPage = "bookList";
    } else if (currentPage === "bookList") {
      currentPage = "home";
    }
  }
</script>

<main class="min-h-screen bg-gradient-to-br from-blue-100 to-purple-100">
  {#if isLoading}
    <div
      class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50"
    >
      <div
        class="loader ease-linear rounded-full border-4 border-t-4 border-gray-200 h-12 w-12"
      ></div>
    </div>
  {/if}

  {#if errorMessage}
    <div
      class="fixed top-4 right-4 bg-red-500 text-white px-4 py-2 rounded-lg shadow-lg z-50 animate-slide-in"
    >
      {errorMessage}
    </div>
  {/if}

  {#if currentPage === "home"}
    <Home onPickPdf={pickAndProcessPdf} />
  {:else if currentPage === "bookList"}
    <BookList {searchResults} onSelectBook={handleBookSelect} onBack={goBack} />
  {:else if currentPage === "bookDetails"}
    <BookDetails
      {bookInfo}
      {selectedBookDetails}
      {jsonOutput}
      onExportJson={exportJson}
      onCopyJson={copyJson}
      onCopyDetailsJson={copyDetailsJson}
      onBack={goBack}
    />
  {/if}
</main>

<style lang="postcss">
  @reference "tailwindcss";
  .loader {
    border-top-color: #3498db;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  .animate-slide-in {
    animation: slideIn 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
</style>
