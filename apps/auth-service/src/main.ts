import { NestFactory } from '@nestjs/core';
import { Logger } from 'nestjs-pino';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  const logger = app.get(Logger);
  const port = process.env.PORT ?? 8080;

  app.useLogger(logger);
  await app.listen(port);

  logger.log(`Auth service is running on port ${port}`, 'Bootstrap');
}
bootstrap().catch(console.error);
