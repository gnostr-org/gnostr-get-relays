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
#endif
