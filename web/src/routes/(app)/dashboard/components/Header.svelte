<script lang="ts">
	import * as Breadcrumb from "$lib/components/breadcrumb"
	import Separator from "$lib/components/Separator.svelte"
	import * as Sidebar from "$lib/components/sidebar"
	import { getBreadcrumbs } from "$lib/utils"
	import { page } from "$app/state"

	let breadcrumbs = $derived(getBreadcrumbs(page.url.pathname, "/dashboard"))
</script>

<header
	class="flex h-16 shrink-0 items-center gap-1.5 px-4 transition-[width,height] ease-linear group-has-data-[state=collapsed]/sidebar-wrapper:h-12 md:px-0"
>
	<Sidebar.Trigger />
	<Separator orientation="vertical" class="me-1.5 data-[orientation=vertical]:h-4" />
	<Breadcrumb.List>
		{#each breadcrumbs as { title, href }, i}
			<Breadcrumb.Item>
				{#if i === breadcrumbs.length - 1}
					<Breadcrumb.Page>{title}</Breadcrumb.Page>
				{:else}
					<Breadcrumb.Link {href}>{title}</Breadcrumb.Link>
				{/if}
			</Breadcrumb.Item>
			{#if i < breadcrumbs.length - 1}
				<Breadcrumb.Separator />
			{/if}
		{/each}
	</Breadcrumb.List>
</header>
