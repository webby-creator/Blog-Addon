/// <reference path="../../../addon-common/ts/index.d.ts" />

interface BlogPostSimpleJson {
    id: number;
    blog_id: number;
    title: string | null;
}

interface BlogPostFullJson {
    id: number;
    blog_id: number;
    title: string;
    content: Delta;
    slug: string | null;
    status: number;
}