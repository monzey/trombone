import { ApiContext } from "@/context/ApiProvider";
import { useContext } from "react";

export const useApi = () => useContext(ApiContext).api
