import { dev } from "$app/environment"
import { defineConfig } from "cva"
import { twMerge } from "tailwind-merge"

export const { cva, cx } = defineConfig({
	hooks: {
		onComplete: (className) => twMerge(className)
	}
})

export const apiUrl = dev ? "http://localhost:8080" : "https://api.lambda.new"
