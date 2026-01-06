import { dev } from "$app/environment"
import { defineConfig } from "cva"
import { twMerge } from "tailwind-merge"
import { MediaQuery } from "svelte/reactivity"

export const { cva, cx } = defineConfig({
	hooks: {
		onComplete: (className) => twMerge(className)
	}
})

export const apiUrl = dev ? "http://localhost:8080" : "https://api.lambda.new"

const DEFAULT_MOBILE_BREAKPOINT = 768

export class IsMobile extends MediaQuery {
	constructor(breakpoint: number = DEFAULT_MOBILE_BREAKPOINT) {
		super(`max-width: ${breakpoint - 1}px`)
	}
}
