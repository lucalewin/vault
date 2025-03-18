export const login = () => {

}

export const register = () => {
  
}

const isTokenExpired = (token: string): boolean => Date.now() >= (JSON.parse(atob(token.split('.')[1]))).exp * 1000;

export const isLoggedIn = (): Boolean => {
  const apiToken = localStorage.getItem('api_token');
  return apiToken != null && !isTokenExpired(apiToken);
}
