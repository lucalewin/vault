import { renderComponent } from "$lib/components/ui/data-table";
import type { ColumnDef } from "@tanstack/table-core";
import DataTableFavicon from "./data-table-favicon.svelte";
import DataTableActions from "./data-table-actions.svelte";

export type PwCredential = {
  id: string;
  service: string;
  username: string;
  password: string | null;
};

export const columns: ColumnDef<PwCredential>[] = [
  {
    id: "favicon",
    cell: ({ row }) => {
      // You can pass whatever you need from `row.original` to the component
      return renderComponent(DataTableFavicon, { url: row.original.service });
    },
  },
  {
    accessorKey: "service",
    header: "Service",
  },
  {
    accessorKey: "username",
    header: "Username",
  },
  // {
  //   accessorKey: "password",
  //   header: "Password",
  // },
  {
    id: "actions",
    cell: ({ row }) => {
      // You can pass whatever you need from `row.original` to the component
      return renderComponent(DataTableActions, { id: row.original.id });
    },
  },
];
