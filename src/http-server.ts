import express from "express";
import cors from "cors";
import { MeetingStatus } from "./types";

export const startHttpServer = (getStatus: () => Promise<MeetingStatus>) => {
  const app = express();

  app.use(
    cors({
      origin: "*",
    }),
  );

  app.use(express.raw({ type: "*/*" }));

  app.use((req, _, next) => {
    console.log(new Date().toISOString(), req.method, req.url);
    next();
  });

  app.get("/led-status", async (_, res) => {
    return res.status(200).send(await getStatus());
  });

  const server = app.listen(80, () => {
    console.log("Server started at port 80");
  });

  process.on("SIGINT", () => {
    server.close();
  });
};
