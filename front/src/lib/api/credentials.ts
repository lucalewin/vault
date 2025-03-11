import { download } from "../util";

export async function new_credential(service: string, username: string, password: string, master_password: string) {
  const token = localStorage.getItem('api_token');
  const response = await fetch('/api/v1/credentials', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      service,
      username,
      password,
      master_password,
    }),
  });

  if (!response.ok) {
    throw new Error('Network response was not ok');
  }

  return await response.json();
}

export async function export_passwords(master_password: string) {
  const api_token = localStorage.getItem('api_token');
  const response = await fetch(`/api/v1/credentials/export`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${api_token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ master_password }),
  });

  const credentials = await response.json();

  download("credentials.json", JSON.stringify(credentials, null, 2))
}
