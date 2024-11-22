from typing import Optional
from datetime import datetime, timezone
from pydantic import BaseModel, EmailStr, Field
from beanie import Document, Indexed
from passlib.context import CryptContext

# Passlib context for password hashing
pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")


# Base User model for common methods and attributes
class BaseUser(Document):
    username: str
    email: Indexed(EmailStr, unique=True)
    password: str  # Hashed
    is_active: bool = Field(default=True)
    is_staff: bool = Field(default=False)
    is_superuser: bool = Field(default=False)

    class Settings:
        name = "users"

    def __str__(self) -> str:
        return self.username

    def has_perm(self, perm: Optional[str] = None, obj: Optional[BaseModel] = None) -> bool:
        return self.is_superuser

    def has_module_perms(self, app_label: str) -> bool:
        return self.is_superuser

    @staticmethod
    def hash_password(password: str) -> str:
        return pwd_context.hash(password)

    def verify_password(self, password: str) -> bool:
        return pwd_context.verify(password, self.password)


# User model with additional fields
class User(BaseUser):
    image: Optional[str] = None
    bio: Optional[str] = None
    birth_date: Optional[datetime] = None
    created_at: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))
    updated_at: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))

    @classmethod
    async def update_user(cls, user_id: str, **kwargs):
        user = await cls.get(user_id)
        if not user:
            raise ValueError("User not found")
        for key, value in kwargs.items():
            setattr(user, key, value)
        user.updated_at = datetime.now(timezone.utc)
        await user.save_changes()
        return user
