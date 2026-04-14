import type { components } from "@shiori/api-client";
import { type ColumnDef } from "@tanstack/table-core";
import { createRawSnippet } from "svelte";
import { renderComponent, renderSnippet } from "../ui/data-table";
import { formatDistanceToNow, intlFormat } from "date-fns";

import DataTableActions from "./data-table-actions.svelte"

export type Token = components["schemas"]["ApiToken"]

export const columns: ColumnDef<Token>[] = [
  {
    accessorKey: "key_id",
    header: "Key ID"
  },
  {
    accessorKey: "name",
    header: "Name"
  },
  {
    accessorKey: "expires_at",
    header: "Expires",
    cell: ({ row }) => {
      const createdAtSnippet = createRawSnippet<[{ date: string | null | undefined }]>(
        (getDate) => {
          const { date } = getDate()

          let formatted: string

          if (!date) {
            formatted = "Never"
          } else {
            const d = new Date(date)
            formatted = d.getTime() < Date.now() ? "Expired" : intlFormat(d, { dateStyle: "long" })
          }

          return {
            render: () => `<div>${formatted}</div>`
          }
        }
      )

      return renderSnippet(createdAtSnippet, { date: row.original.expires_at })
    }
  },
  {
    accessorKey: "created_at",
    header: "Created",
    cell: ({ row }) => {
      const formatter = new Intl.DateTimeFormat("en-us", { dateStyle: "long", timeStyle: "short" })

      const createdAtSnippet = createRawSnippet<[{ date: string }]>(
        (getDate) => {
          const { date } = getDate()
          const formatted = formatter.format(new Date(date))
          return {
            render: () => `<div>${formatted}</div>`
          }
        }
      )

      return renderSnippet(createdAtSnippet, { date: row.original.created_at })
    }
  },
  {
    accessorKey: "last_used_at",
    header: "Last Used",
    cell: ({ row }) => {
      const createdAtSnippet = createRawSnippet<[{ date: string | null | undefined }]>(
        (getDate) => {
          const { date } = getDate()
          const formatted = date ? formatDistanceToNow(date, { addSuffix: true }) : "Never Used"
          return {
            render: () => `<div>${formatted}</div>`
          }
        }
      )

      return renderSnippet(createdAtSnippet, { date: row.original.last_used_at })
    }
  },
  {
    id: "actions",
    cell: ({ row }) => {
      return renderComponent(DataTableActions, { key_id: row.original.key_id });
    },
  },
]



