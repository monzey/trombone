import axios, { type AxiosInstance } from "axios";
import { createContext, type FC, useCallback, type PropsWithChildren } from "react";
import useSWR, { SWRConfig, type SWRResponse } from "swr";

type Api = AxiosInstance & {
  fetch: <T = any>(
    key: string | null,
    fetcher?: (url: string) => Promise<T>,
    config?: any
  ) => SWRResponse<T, any>;
};

interface ApiContextType {
  api: Api;
}

interface ApiProps { }

const defaultApi = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
}) as Api;

export const ApiContext = createContext<ApiContextType>({
  api: defaultApi,
});

export const ApiProvider: FC<PropsWithChildren<ApiProps>> = ({ children, ...props }) => {
  const api = axios.create({
    baseURL: import.meta.env.VITE_API_BASE_URL,
    headers: {
      "Content-Type": "application/json",
    },
  }) as Api;
  api.fetch = useSWR

  const fetcher = useCallback(async (url: string, ...args: any[]) => {
    const res = await api.get(url, ...args)

    return res.data
  }, [])

  return <ApiContext.Provider value={{ api }} {...props}>
    <SWRConfig value={{ fetcher, revalidateOnFocus: false, use: [] }}>
      {children}
    </SWRConfig>
  </ApiContext.Provider>
}
