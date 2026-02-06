<script lang="ts">
	import { goto } from "$app/navigation"
	import type { LayoutProps } from "./$types"

	let { data, children }: LayoutProps = $props()

	$effect(() => {
		!data.user && goto("/login", { replaceState: true })
	})
</script>

{@render children()}
