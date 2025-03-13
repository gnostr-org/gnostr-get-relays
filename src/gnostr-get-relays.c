#ifndef GNOSTR_GET_RELAYS
#define GNOSTR_GET_RELAYS
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
void help(){
  printf("help");
    exit(0);
}
void version(){
  printf("v0.0.0");
    exit(0);
}
int gnostr_get_relays(int size) {
  char command[size];
  strcpy(command, "curl  -sS 'https://api.nostr.watch/v1/online' > /tmp/gnostr.relays ");
  system(command);
  strcpy(command, "echo $(cat /tmp/gnostr.relays | sed 's/\\[//' | sed 's/\\]//' | sed 's/\"//g')");
  system(command);
  return 0;
}
char* read_file_to_string(const char* filename) {
    FILE* file = fopen(filename, "r");
    if (file == NULL) {
        perror("Error opening file");
        return NULL;
    }

    // Determine file size.
    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    fseek(file, 0, SEEK_SET); // Rewind to beginning.

    if (file_size < 0) {
        perror("Error determining file size");
        fclose(file);
        return NULL;
    }

    // Allocate memory for the file content (+1 for null terminator).
    char* buffer = (char*)malloc(file_size + 1);
    if (buffer == NULL) {
        perror("Memory allocation failed");
        fclose(file);
        return NULL;
    }

    // Read the file content into the buffer.
    size_t read_size = fread(buffer, 1, file_size, file);
    if (read_size != (size_t)file_size) {
        if (ferror(file)) {
            perror("Error reading file");
        } else {
            fprintf(stderr, "Unexpected end of file.\n");
        }
        free(buffer);
        fclose(file);
        return NULL;
    }

    buffer[file_size] = '\0'; // Null-terminate the string.
    fclose(file);

    return buffer;
}

char* get_list() {
    const char* filename = "/tmp/gnostr.relays"; // Replace with your filename.

    // Create a dummy file for testing, if it doesn't already exist.
    FILE* test_file = fopen(filename, "r");
    if(test_file == NULL){
        test_file = fopen(filename, "w");
        if(test_file != NULL){
            fprintf(test_file, "This is a test file.\nAnother line.");
            fclose(test_file);
        }
    } else {
        fclose(test_file);
    }

    char* file_content = read_file_to_string(filename);

    if (file_content != NULL) {
        printf("%s", file_content);
        free(file_content); // Free the allocated memory.
    } else {
        printf("Failed to read file.\n");
    }

    return file_content;
}
#endif
