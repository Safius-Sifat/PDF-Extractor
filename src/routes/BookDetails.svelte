<script lang="ts">
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { save } from "@tauri-apps/plugin-dialog";
  import { writeTextFile } from "@tauri-apps/plugin-fs";

  export let selectedBookDetails: { [key: string]: any } | null;
  export let onCopyDetailsJson: () => void;
  export let onBack: () => void;

  let bookDetails: any = {};
  let toastMessage = "";
  let showToast = false;

  $: if (selectedBookDetails) {
    bookDetails = transformGoogleBookJson(selectedBookDetails);
  }

  function slugify(text: string): string {
    return text
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-|-$/g, "");
  }

  function transformGoogleBookJson(book: any) {
    const volumeInfo = book.volumeInfo || {};
    const title = volumeInfo.title || "Unknown Title";
    const authors = volumeInfo.authors || ["Unknown Author"];
    const publisher = volumeInfo.publisher || "Unknown Publisher";
    const description = volumeInfo.description
      ? volumeInfo.description.replace(/<[^>]+>/g, "") // Strip HTML tags
      : "No description available.";
    const isbn =
      volumeInfo.industryIdentifiers?.find((id: any) => id.type === "ISBN_13")
        ?.identifier || "BOOK000000";
    const categories = volumeInfo.categories
      ? volumeInfo.categories.map(
          (cat: string) => cat.split("/").pop()?.trim() || "Uncategorized",
        )
      : ["Uncategorized"];
    const thumbnail =
      volumeInfo.imageLinks?.thumbnail || `${slugify(title)}_cover.jpg`;

    const bookSlug = slugify(title);

    return {
      name: title,
      sku: isbn,
      type: "variable",
      image: thumbnail,
      images: [
        `${bookSlug}_image1.jpg`,
        `${bookSlug}_image2.jpg`,
        `${bookSlug}_image3.jpg`,
      ],
      preview: `${bookSlug}_preview.pdf`,
      description,
      author: authors.map((name: string) => ({
        slug: slugify(name),
        name,
      })),
      category: categories.map((name: string) => ({
        slug: slugify(name),
        name,
      })),
      tag: [
        { slug: "bestseller", name: "Bestseller" },
        { slug: "new-arrival", name: "New Arrival" },
      ],
      publisher: [{ slug: slugify(publisher), name: publisher }],
      prices_n_stock: [
        {
          print: "Paperback",
          cover: "Softcover",
          purchase_price: "200",
          sell_price: "300",
          is_discount: true,
          discount_type: "percentage",
          discount_amount: "10",
          after_discount: "270",
          stock_qty: "100",
        },
        {
          print: "Hardcover",
          cover: "Hardback",
          purchase_price: "400",
          sell_price: "500",
          is_discount: false,
          discount_type: null,
          discount_amount: "0",
          after_discount: "500",
          stock_qty: "50",
        },
      ],
    };
  }

  function addAuthor() {
    bookDetails.author = [...bookDetails.author, { slug: "", name: "" }];
  }

  function addCategory() {
    bookDetails.category = [...bookDetails.category, { slug: "", name: "" }];
  }

  function addTag() {
    bookDetails.tag = [...bookDetails.tag, { slug: "", name: "" }];
  }

  function addPublisher() {
    bookDetails.publisher = [...bookDetails.publisher, { slug: "", name: "" }];
  }

  function addPriceStock() {
    bookDetails.prices_n_stock = [
      ...bookDetails.prices_n_stock,
      {
        print: "Paperback",
        cover: "Softcover",
        purchase_price: "0",
        sell_price: "0",
        is_discount: false,
        discount_type: null,
        discount_amount: "0",
        after_discount: "0",
        stock_qty: "0",
      },
    ];
  }

  function removeItem(array: any[], index: number) {
    return array.filter((_, i) => i !== index);
  }

  async function copyJson() {
    try {
      const json = JSON.stringify(bookDetails, null, 2);
      await writeText(json);
      toastMessage = "Transformed JSON copied to clipboard!";
      showToast = true;
    } catch (error) {
      toastMessage = `Error copying JSON: ${error}`;
      showToast = true;
    }
  }

  async function exportJson() {
    try {
      const filePath = await save({
        filters: [{ name: "JSON", extensions: ["json"] }],
        defaultPath: "book_details.json",
      });
      if (filePath) {
        const json = JSON.stringify(bookDetails, null, 2);
        await writeTextFile(filePath, json);
        toastMessage = "Transformed JSON exported successfully!";
        showToast = true;
      }
    } catch (error) {
      toastMessage = `Error exporting JSON: ${error}`;
      showToast = true;
    }
  }

  // Inline Toast (since Toast.svelte isn't provided)
  function showToastMessage(message: string) {
    toastMessage = message;
    showToast = true;
    setTimeout(() => (showToast = false), 3000);
  }
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

  {#if selectedBookDetails && bookDetails.name}
    <div class="bg-white rounded-lg shadow-md p-6">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-xl font-semibold text-gray-800">
          Detailed Book Information
        </h3>
        <div class="space-x-2">
          <button
            on:click={onCopyDetailsJson}
            class="bg-purple-600 text-white px-4 py-2 rounded-lg hover:bg-purple-700 transition duration-300 cursor-pointer"
          >
            Copy Original JSON
          </button>
          <button
            on:click={copyJson}
            class="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition duration-300 cursor-pointer"
          >
            Copy Transformed JSON
          </button>
          <button
            on:click={exportJson}
            class="bg-green-600 text-white px-4 py-2 rounded-lg hover:bg-green-700 transition duration-300 cursor-pointer"
          >
            Export Transformed JSON
          </button>
        </div>
      </div>

      <div class="space-y-4">
        <div>
          <label class="block font-semibold text-gray-700">Name</label>
          <input
            type="text"
            class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            bind:value={bookDetails.name}
          />
        </div>
        <div>
          <label class="block font-semibold text-gray-700">SKU</label>
          <input
            type="text"
            class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            bind:value={bookDetails.sku}
          />
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Type</label>
          <input
            type="text"
            class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            bind:value={bookDetails.type}
          />
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Image</label>
          <input
            type="text"
            class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            bind:value={bookDetails.image}
          />
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Images</label>
          {#each bookDetails.images as image, i}
            <div class="flex space-x-2 mb-2">
              <input
                type="text"
                class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value={bookDetails.images[i]}
              />
              <button
                class="p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
                on:click={() =>
                  (bookDetails.images = removeItem(bookDetails.images, i))}
              >
                -
              </button>
            </div>
          {/each}
          <button
            class="p-2 bg-green-500 text-white rounded-lg hover:bg-green-600"
            on:click={() =>
              (bookDetails.images = [
                ...bookDetails.images,
                `${slugify(bookDetails.name)}_image${bookDetails.images.length + 1}.jpg`,
              ])}
          >
            + Add Image
          </button>
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Preview</label>
          <input
            type="text"
            class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            bind:value={bookDetails.preview}
          />
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Description</label>
          <textarea
            class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            bind:value={bookDetails.description}
          ></textarea>
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Authors</label>
          {#each bookDetails.author as author, i}
            <div class="flex space-x-2 mb-2">
              <input
                type="text"
                placeholder="Slug"
                class="w-1/2 p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value={bookDetails.author[i].slug}
              />
              <input
                type="text"
                placeholder="Name"
                class="w-1/2 p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value={bookDetails.author[i].name}
              />
              <button
                class="p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
                on:click={() =>
                  (bookDetails.author = removeItem(bookDetails.author, i))}
              >
                -
              </button>
            </div>
          {/each}
          <button
            class="p-2 bg-green-500 text-white rounded-lg hover:bg-green-600"
            on:click={addAuthor}
          >
            + Add Author
          </button>
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Categories</label>
          {#each bookDetails.category as category, i}
            <div class="flex space-x-2 mb-2">
              <input
                type="text"
                placeholder="Slug"
                class="w-1/2 p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value={bookDetails.category[i].slug}
              />
              <input
                type="text"
                placeholder="Name"
                class="w-1/2 p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value={bookDetails.category[i].name}
              />
              <button
                class="p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
                on:click={() =>
                  (bookDetails.category = removeItem(bookDetails.category, i))}
              >
                -
              </button>
            </div>
          {/each}
          <button
            class="p-2 bg-green-500 text-white rounded-lg hover:bg-green-600"
            on:click={addCategory}
          >
            + Add Category
          </button>
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Tags</label>
          {#each bookDetails.tag as tag, i}
            <div class="flex space-x-2 mb-2">
              <input
                type="text"
                placeholder="Slug"
                class="w-1/2 p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value={bookDetails.tag[i].slug}
              />
              <input
                type="text"
                placeholder="Name"
                class="w-1/2 p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value={bookDetails.tag[i].name}
              />
              <button
                class="p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
                on:click={() =>
                  (bookDetails.tag = removeItem(bookDetails.tag, i))}
              >
                -
              </button>
            </div>
          {/each}
          <button
            class="p-2 bg-green-500 text-white rounded-lg hover:bg-green-600"
            on:click={addTag}
          >
            + Add Tag
          </button>
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Publishers</label>
          {#each bookDetails.publisher as publisher, i}
            <div class="flex space-x-2 mb-2">
              <input
                type="text"
                placeholder="Slug"
                class="w-1/2 p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value={bookDetails.publisher[i].slug}
              />
              <input
                type="text"
                placeholder="Name"
                class="w-1/2 p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                bind:value={bookDetails.publisher[i].name}
              />
              <button
                class="p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
                on:click={() =>
                  (bookDetails.publisher = removeItem(
                    bookDetails.publisher,
                    i,
                  ))}
              >
                -
              </button>
            </div>
          {/each}
          <button
            class="p-2 bg-green-500 text-white rounded-lg hover:bg-green-600"
            on:click={addPublisher}
          >
            + Add Publisher
          </button>
        </div>
        <div>
          <label class="block font-semibold text-gray-700">Prices & Stock</label
          >
          {#each bookDetails.prices_n_stock as price, i}
            <div class="border p-4 mb-2 rounded space-y-2">
              <div class="flex space-x-2">
                <div class="w-1/2">
                  <label class="block text-gray-600">Print</label>
                  <input
                    type="text"
                    class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    bind:value={bookDetails.prices_n_stock[i].print}
                  />
                </div>
                <div class="w-1/2">
                  <label class="block text-gray-600">Cover</label>
                  <input
                    type="text"
                    class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    bind:value={bookDetails.prices_n_stock[i].cover}
                  />
                </div>
              </div>
              <div class="flex space-x-2">
                <div class="w-1/2">
                  <label class="block text-gray-600">Purchase Price</label>
                  <input
                    type="text"
                    class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    bind:value={bookDetails.prices_n_stock[i].purchase_price}
                  />
                </div>
                <div class="w-1/2">
                  <label class="block text-gray-600">Sell Price</label>
                  <input
                    type="text"
                    class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    bind:value={bookDetails.prices_n_stock[i].sell_price}
                  />
                </div>
              </div>
              <div class="flex space-x-2">
                <div class="w-1/2">
                  <label class="block text-gray-600">Is Discount</label>
                  <input
                    type="checkbox"
                    class="p-2"
                    bind:checked={bookDetails.prices_n_stock[i].is_discount}
                  />
                </div>
                <div class="w-1/2">
                  <label class="block text-gray-600">Discount Type</label>
                  <input
                    type="text"
                    class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    bind:value={bookDetails.prices_n_stock[i].discount_type}
                    disabled={!bookDetails.prices_n_stock[i].is_discount}
                  />
                </div>
              </div>
              <div class="flex space-x-2">
                <div class="w-1/2">
                  <label class="block text-gray-600">Discount Amount</label>
                  <input
                    type="text"
                    class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    bind:value={bookDetails.prices_n_stock[i].discount_amount}
                  />
                </div>
                <div class="w-1/2">
                  <label class="block text-gray-600">After Discount</label>
                  <input
                    type="text"
                    class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    bind:value={bookDetails.prices_n_stock[i].after_discount}
                  />
                </div>
              </div>
              <div>
                <label class="block text-gray-600">Stock Quantity</label>
                <input
                  type="text"
                  class="w-full p-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  bind:value={bookDetails.prices_n_stock[i].stock_qty}
                />
              </div>
              <button
                class="p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
                on:click={() =>
                  (bookDetails.prices_n_stock = removeItem(
                    bookDetails.prices_n_stock,
                    i,
                  ))}
              >
                - Remove
              </button>
            </div>
          {/each}
          <button
            class="p-2 bg-green-500 text-white rounded-lg hover:bg-green-600"
            on:click={addPriceStock}
          >
            + Add Price & Stock
          </button>
        </div>
        <div>
          <label class="block font-semibold text-gray-700">JSON Preview</label>
          <pre
            class="bg-gray-100 p-4 rounded max-h-120 overflow-auto">{JSON.stringify(
              bookDetails,
              null,
              2,
            )}</pre>
        </div>
      </div>
    </div>
  {:else}
    <p class="text-gray-600">No book details available.</p>
  {/if}

  {#if showToast}
    <div
      class="fixed bottom-4 right-4 bg-blue-500 text-white px-4 py-2 rounded-lg shadow-lg"
    >
      {toastMessage}
      <button class="ml-2 text-white" on:click={() => (showToast = false)}>
        ×
      </button>
    </div>
  {/if}
</div>

<style>
  input,
  textarea {
    border-color: #e2e8f0;
  }
  button {
    transition: background-color 0.2s;
  }
  button:hover {
    opacity: 0.9;
  }
  pre {
    white-space: pre-wrap;
  }
</style>
