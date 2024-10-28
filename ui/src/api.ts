declare global {
    interface Window {
        conduct: {
            get: (path: string) => Promise<Response>
        }
    }
}

export function get(path: string) {
    return window.conduct.get(path)
}