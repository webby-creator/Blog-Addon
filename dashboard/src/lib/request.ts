import type { Delta } from "quill/core";

export async function getOverview(): Promise<unknown> {
    return fetchJson(compApiUrl(`/blog/${INSTANCE_UUID}/overview`), { method: 'GET' });
}

export async function getPostList(): Promise<ListResponse<BlogPostSimpleJson>> {
    return fetchJson(compApiUrl(`/blog/${INSTANCE_UUID}/posts`), { method: 'GET' });
}

export async function createPost(title: string, content: Delta): Promise<BlogPostFullJson> {
    return fetchJson(
        compApiUrl(`/blog/${INSTANCE_UUID}/post`),
        {
            method: 'POST',
            body: JSON.stringify({ title, content }),
            headers: {
                'Content-Type': 'application/json'
            }
        }
    );
}

export async function updatePost(
    id: number,
    opts: { title?: string; content?: Delta; slug?: string; status?: number; }
): Promise<string> {
    return fetchJson(
        compApiUrl(`/blog/${INSTANCE_UUID}/post/${id}`),
        {
            method: 'POST',
            body: JSON.stringify(opts),
            headers: {
                'Content-Type': 'application/json'
            }
        }
    );
}

export async function getPost(
    id: number
): Promise<BlogPostFullJson> {
    return fetchJson(
        compApiUrl(`/blog/${INSTANCE_UUID}/post/${id}`),
        { method: 'GET' }
    );
}


export type JsonResponse<V> = {
    type: "Resp";
    value: V;
} | { type: "Error"; value: { description: string; }; };

export type ListResponse<T> = {
    items: T[];
    offset: number;
    limit: number;
    total: number;
};


export function compApiUrl(path: string) {
    return `${API_URL}${path}`;
}

export async function fetchJsonBody<V, B>(input: RequestInfo | URL, init: RequestInit = {}, body: B): Promise<V> {
    return fetchJson(
        input,
        Object.assign(init, {
            body: JSON.stringify(body)
        })
    );
}

export async function fetchJson<V>(input: RequestInfo | URL, init: RequestInit = {}): Promise<V> {
    init.mode = 'cors';
    init.credentials = 'include';

    if (init.body != null) {
        init.headers = {
            // "Referrer-Policy": "no-referrer-when-downgrade",
            "Content-Type": "application/json",
        };
    }

    const resp = await fetch(input, init);
    const json: JsonResponse<V> = await resp.json();

    if (json.type == 'Resp') {
        return json.value;
    } else {
        throw new Error(json.value.description);
    }
}

export async function fetchJsonRaw<V>(input: RequestInfo | URL, init: RequestInit = {}): Promise<V> {
    init.mode = 'cors';
    init.credentials = 'include';

    if (init.body != null) {
        init.headers = {
            // "Referrer-Policy": "no-referrer-when-downgrade",
            "Content-Type": "application/json",
        };
    }

    const resp = await fetch(input, init);

    return await resp.json();
}

export async function fetchString(input: RequestInfo | URL, init: RequestInit = {}): Promise<string> {
    init.mode = 'cors';
    init.credentials = 'include';

    if (init.body != null) {
        init.headers = {
            // "Referrer-Policy": "no-referrer-when-downgrade",
            "Content-Type": "application/json",
        };
    }

    const resp = await fetch(input, init);

    return await resp.text();
}