<!-- <svelte:options customElement="dash-embed" /> -->

<script lang="ts">
    import type { Component } from "svelte";

    import rootRoute from "./routes/+page.svelte";
    import analyticsRoute from "./routes/analytics/+page.svelte";
    import categoriesRoute from "./routes/categories/+page.svelte";
    import commentsRoute from "./routes/comments/+page.svelte";
    import createPostRoute from "./routes/create-post/+page.svelte";
    import viewPostRoute from "./routes/view-post/+page.svelte";
    import postsRoute from "./routes/posts/+page.svelte";
    import tagsRoute from "./routes/tags/+page.svelte";

    let { path }: { path: string } = $props();

    const baseRoutes: {
        [path: string]: { page?: Component; length?: number };
    } = {
        overview: { page: rootRoute },
        analytics: { page: analyticsRoute },
        categories: { page: categoriesRoute },
        comments: { page: commentsRoute },
        "create-post": { page: createPostRoute },
        posts: { page: postsRoute },
        tags: { page: tagsRoute },
        "view-post": { page: viewPostRoute, length: 1 },
    };

    const split = path.split("/").slice(1);
    const found = baseRoutes[split[0]] ?? { page: undefined };

    const Route =
        found.length != null
            ? found.length == split.length - 1
                ? found.page
                : undefined
            : found.page;
</script>

<main>
    {#if Route != null}
        <Route />
    {/if}
</main>
