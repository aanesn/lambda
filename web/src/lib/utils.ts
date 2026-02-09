import { defineConfig } from "cva"
import { MediaQuery } from "svelte/reactivity"
import { twMerge } from "tailwind-merge"
import { dev } from "$app/environment"

export const { cva, cx } = defineConfig({
	hooks: {
		onComplete: (className) => twMerge(className)
	}
})

export const apiUrl = dev ? "http://localhost:8080" : "https://api.lambda.new/"

const DEFAULT_MOBILE_BREAKPOINT = 768

export class IsMobile extends MediaQuery {
	constructor(breakpoint: number = DEFAULT_MOBILE_BREAKPOINT) {
		super(`max-width: ${breakpoint - 1}px`)
	}
}

export function getBreadcrumbs(pathname: string, baseRoute = "/dashboard") {
	const segments = pathname.split("/").filter(Boolean).slice(1)
	return segments.map((segment, i) => ({
		title: segment.charAt(0).toUpperCase() + segment.slice(1).replaceAll("-", " "),
		href: `${baseRoute}/${segments.slice(0, i + 1).join("/")}`
	}))
}
