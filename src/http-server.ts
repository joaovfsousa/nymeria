import express from "express";
import cors from "cors";
import { MeetingStatus } from "./types";

export const startHttpServer = (getStatus: () => MeetingStatus) => {
  const app = express();

  app.use(
    cors({
      origin: "*",
    }),
  );

  app.use(express.raw({ type: "*/*" }));

  app.use(async (req, _, next) => {
    console.log("Request: ", new Date().toISOString(), req.method, req.url);
    next();
  });

  app.get("/led-status", async (req, res) => {
    const status = getStatus();
    console.log(
      `Response: ${status}`,
      new Date().toISOString(),
      req.method,
      req.url,
    );
    return res.status(200).send(status);
  });

  const server = app.listen(80, () => {
    console.log("Server started at port 80");
  });

  process.on("SIGINT", () => {
    server.close();
  });
};
